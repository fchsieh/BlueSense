mod bluetooth;

extern crate lru;

use std::error::Error;
use std::num::NonZeroUsize;
use btleplug::platform::Manager;

use std::sync::{ Arc, Mutex };

const MAX_CACHE_SIZE: usize = 100;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cache = Arc::new(
        Mutex::new(lru::LruCache::new(NonZeroUsize::new(MAX_CACHE_SIZE).unwrap()))
    );

    pretty_env_logger::init();

    // start finding bluetooth devices
    let manager = Manager::new().await?;
    let (_, _) = tokio::join!(
        bluetooth::start_discover(&manager, cache.clone()),
        bluetooth::report_device_count(cache.clone())
    );

    Ok(())
}