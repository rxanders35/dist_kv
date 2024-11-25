use std::sync::RwLock;
use std::collections::HashMap;
pub struct Cache {
    data: RwLock<HashMap<Vec<u8>, String>>
}

impl Cache {
    pub fn new() -> Self {
        Cache {
            data: RwLock::new(HashMap::new()),
        } 
    }
}