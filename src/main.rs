extern crate lru;

use std::error::Error;
use std::num::NonZeroUsize;
use std::time::{ SystemTime, UNIX_EPOCH };
use std::sync::{ Arc, Mutex };
use futures::stream::StreamExt;
use btleplug::api::{ Central, CentralEvent, Manager as _, ScanFilter };
use btleplug::platform::{ Adapter, Manager, PeripheralId };

const MAX_CACHE_SIZE: usize = 100;
const REPORT_INTERVAL: u64 = 5;
type Cache = Arc<Mutex<lru::LruCache<String, u128>>>;

async fn get_central(manager: &Manager) -> Adapter {
    let adapters = manager.adapters().await.unwrap();
    adapters.into_iter().nth(0).unwrap()
}

async fn on_device_discovered(id: &PeripheralId, cache: Cache) {
    let address = id.to_string();
    // update LRU cache by timestamp
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let mut cache = cache.lock().unwrap();
    cache.put(address, now);
}

async fn start_discover(manager: &Manager, cache: Cache) -> Result<(), Box<dyn Error>> {
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

async fn report_device_count(cache: Cache) {
    loop {
        println!("Cache size: {}", cache.lock().unwrap().len());
        tokio::time::sleep(tokio::time::Duration::from_secs(REPORT_INTERVAL)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //let mut cache = lru::LruCache::new(NonZeroUsize::new(MAX_CACHE_SIZE).unwrap());
    let cache = Arc::new(
        Mutex::new(lru::LruCache::new(NonZeroUsize::new(MAX_CACHE_SIZE).unwrap()))
    );

    pretty_env_logger::init();

    // start finding bluetooth devices
    let manager = Manager::new().await?;
    let (_, _) = tokio::join!(
        start_discover(&manager, cache.clone()),
        report_device_count(cache.clone())
    );

    Ok(())
}