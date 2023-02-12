mod bluetooth;
mod http;

extern crate lru;
extern crate pretty_env_logger;

use btleplug::platform::Manager;
use std::error::Error;
use std::num::NonZeroUsize;
use std::sync::{ Arc, Mutex };
use toml;

const MAX_CACHE_SIZE: usize = 100;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let config = toml::from_str(include_str!("../config.toml"))?;

    let cache = Arc::new(
        Mutex::new(lru::LruCache::new(NonZeroUsize::new(MAX_CACHE_SIZE).unwrap()))
    );

    // spawn tasks
    let manager = Manager::new().await?;
    let _ = tokio::join!(
        bluetooth::start_discover(&manager, cache.clone()),
        bluetooth::report_device_count(cache.clone()),
        http::http_server(&config)
    );

    Ok(())
}