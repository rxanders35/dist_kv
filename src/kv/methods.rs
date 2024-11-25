use std::time;
use dashmap::DashMap;

pub trait CacheOperations {
    fn set(&self, key: Vec<u8>, value: Vec<u8>, ttl: time::Duration) -> Result<(), String>;
    fn has(&self, key: Vec<u8>) -> bool;
    fn get(&self, key: &[u8]) -> Result<Vec<u8>, String>;
    fn delete(&self, key: Vec<u8>) -> Result<(),String>;
}

pub struct Cache {
    data: DashMap<Vec<u8>, Vec<u8>>
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            data: DashMap::new()
        } 
    }
}

impl CacheOperations for Cache {
    fn set(&self, key: Vec<u8>, value: Vec<u8>, ttl: time::Duration) -> Result<(), String> {
        self.data.insert(key, value);
        Ok(())
    }
    fn get(&self, key: &[u8]) -> Result<Vec<u8>, String> {
        self.data.get(key)
        .map(|v| v.to_vec())
        .ok_or_else(|| "Key not found".to_string())
    }

    fn has(&self, key: Vec<u8>) -> bool {
        self.data.contains_key(&key)
    }
    
    fn delete(&self, key: Vec<u8>) -> Result<(), String>{
        Ok(())
    } 
}