use crate::cache::memory::Storage;
use std::collections::HashMap;

pub struct Data {
    pub hash: HashMap<String, String>,
    pub storage: Storage,
}

impl Data {
    pub fn new() -> Self {
        Self {
            hash: HashMap::new(),
            storage: Storage::new("cache.txt"),
        }
    }
    pub fn write(&mut self, operation: String, key: String, val: String) {
        self.hash.insert(key.clone(), val.clone());
        self.storage.add(&operation, &key, &val);
    }
    pub fn read(&mut self, key: String) {
        let value = self.hash.get(&key);
        match value {
            Some(val) => println!("{:?}", val),
            None => println!("Empty"),
        }
    }
}
