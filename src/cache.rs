use lazy_static::lazy_static;
use serde::Serialize;
use time::OffsetDateTime;
use time::ext::NumericalDuration;
use std::{sync::Mutex, collections::HashMap};

pub struct CacheEntry {
  response: String,
  expiring: i64,
}

lazy_static! {
    static ref CACHE: Mutex<HashMap<String, CacheEntry>> = Mutex::new(HashMap::new());
}

pub fn read<T: Serialize>(key: &T) -> Option<String> {
    let cache = CACHE.lock().unwrap();
    let cache_key = serde_json::to_string(key).unwrap();
    match cache.get(cache_key.as_str()) {
        Some(entry) => {
            if entry.expiring > OffsetDateTime::now_utc().unix_timestamp() {
                return Some(entry.response.clone());
            }
            return None;
        }
        None => {None},
    }
}


pub fn write<T: Serialize>(key: &T, response: &str, ttl: i64) {
    let mut cache = CACHE.lock().unwrap();
    let cache_key = serde_json::to_string(key).unwrap();
    let expiring = OffsetDateTime::now_utc()
        .checked_add(ttl.seconds())
        .unwrap()
        .unix_timestamp();

    cache.insert(
        cache_key,
        CacheEntry {
            expiring,
            response: response.to_string(),
        },
    );
}
