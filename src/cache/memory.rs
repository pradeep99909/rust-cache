use std::collections::HashMap;

pub struct Data {
    pub hash: HashMap<String, String>
}

impl Data {
    pub fn add(&mut self, key: String, val: String) {
        self.hash.insert(key, val);
    }
    pub fn read(&mut self, key: String) {
        let value = self.hash.get(&key);
        match value {
            Some(val) => println!("{:?}",val),
            None => println!("Empty")
        }
    }
}