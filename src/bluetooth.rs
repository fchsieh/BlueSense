use std::error::Error;
use std::time::{ SystemTime, UNIX_EPOCH };
use futures::stream::StreamExt;
use btleplug::api::{ Central, CentralEvent, Manager as _, ScanFilter };
use btleplug::platform::{ Adapter, Manager, PeripheralId };
use std::sync::{ Arc, Mutex };

type Cache = Arc<Mutex<lru::LruCache<String, u128>>>;
const REPORT_INTERVAL: u64 = 5;
const EXPIRATION_TIME: u128 = 60 * 1000;

pub async fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().nth(0).unwrap()
}

pub async fn on_device_discovered(id: &PeripheralId, cache: Cache) {
    let address = id.to_string();
    // update LRU cache by timestamp
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let mut cache = cache.lock().unwrap();
    cache.put(address, now);
}

pub async fn start_discover(manager: &Manager, cache: Cache) -> Result<(), Box<dyn Error>> {
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

pub async fn report_device_count(cache: Cache) {
    loop {
        // report device count
        let count = cache.lock().unwrap().len();
        println!("Device count: {}", count);
        tokio::time::sleep(tokio::time::Duration::from_secs(REPORT_INTERVAL)).await;

        // check if least recently used device is expired
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let expired = now - EXPIRATION_TIME;
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