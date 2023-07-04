use std::collections::HashMap;

pub struct Data {
    pub hash: HashMap<String, i8>
}

impl Data {
    pub fn add(&mut self, key: String, val: i8) {
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