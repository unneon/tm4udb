use std::collections::HashMap;

pub struct TrivialLocalInMemory {
    values: HashMap<Vec<u8>, Vec<u8>>,
}

impl TrivialLocalInMemory {
    pub fn new() -> TrivialLocalInMemory {
        TrivialLocalInMemory {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<&[u8]> {
        self.values.get(key).map(Vec::as_slice)
    }

    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) {
        self.values.insert(key, value);
    }

    pub fn delete(&mut self, key: &[u8]) {
        self.values.remove(key);
    }
}
