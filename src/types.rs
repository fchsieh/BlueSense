use std::sync::{ Arc, Mutex };

pub type Table = Arc<Mutex<lru::LruCache<String, u128>>>;