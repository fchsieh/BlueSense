#[path = "types.rs"]
mod types;

use btleplug::api::{ Central, CentralEvent, Manager as _, ScanFilter };
use btleplug::platform::{ Adapter, Manager, PeripheralId };
use chrono;
use futures::stream::StreamExt;
use md5;
use std::error::Error;
use std::time::{ SystemTime, UNIX_EPOCH };

pub async fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().nth(0).unwrap()
}

pub async fn on_device_discovered(id: &PeripheralId, cache: types::Table) {
    let address = id.to_string();
    let digest = format!("{:x}", md5::compute(address.as_bytes()));
    // update LRU cache by timestamp
    let now = chrono::offset::Utc::now().timestamp_millis() as u128;
    let mut cache = cache.lock().unwrap();
    cache.put(digest, now);
}

pub async fn start_discover(manager: &Manager, cache: types::Table) -> Result<(), Box<dyn Error>> {
    let central = get_central(manager).await;
    let mut events = central.events().await?;
    // start scanning for devices
    central.start_scan(ScanFilter::default()).await?;
    while let Some(event) = events.next().await {
        match event {
            CentralEvent::DeviceDiscovered(id) => on_device_discovered(&id, cache.clone()).await,
            CentralEvent::DeviceUpdated(id) => on_device_discovered(&id, cache.clone()).await,
            _ => {}
        }
    }
    Ok(())
}

pub async fn report_device_count(config: &toml::value::Table, cache: types::Table) {
    loop {
        let report_interval = config["bluetooth"]["report"].as_integer().unwrap() as u64;
        let expiration_time =
            (config["bluetooth"]["expiration"].as_integer().unwrap() as u128) * 1000;
        // report device count
        let count = cache.lock().unwrap().len();
        println!("Device count: {}", count);
        tokio::time::sleep(tokio::time::Duration::from_secs(report_interval)).await;

        // check if least recently used device is expired
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let expired = now - expiration_time;
        // find expired devices
        let mut cache = cache.lock().unwrap();
        let expired_keys: Vec<String> = cache
            .iter()
            .filter(|(_, &v)| v < expired)
            .map(|(k, _)| k.clone())
            .collect();
        // remove expired devices
        for key in expired_keys {
            println!("Removing expired device: {}", key);
            cache.pop(&key);
        }
    }
}