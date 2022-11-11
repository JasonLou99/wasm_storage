//  基于HashMap的键值存储
use std::{
    collections::{hash_map, HashMap},
    fmt::Error,
};

// use std::time::{SystemTime, UNIX_EPOCH};

pub struct TimestampValue {
    value: Vec<u8>,
    timestamp: u128,
}

impl TimestampValue {
    pub fn merge(&mut self, other: &TimestampValue) {
        if self.timestamp < other.timestamp {
            self.timestamp = other.timestamp;
            self.value = other.value.clone();
        }
    }
}

pub struct Store {
    db: HashMap<Vec<u8>, TimestampValue>,
}

impl Store {
    pub fn new() -> Store {
        Store { db: HashMap::new() }
    }

    pub fn put(&mut self, key: Vec<u8>, value: TimestampValue) -> Result<(), Error> {
        // 判断key是否存在
        match self.db.entry(key) {
            // 该key不存在就直接插入
            hash_map::Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(())
            }
            // 该key已经存在，则merge value
            hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().merge(&value);
                Ok(())
            }
        }
    }

    pub fn get(&self, key: &Vec<u8>) {
        self.db.get(key);
    }
}

// 获得当前时间戳
// SystemTime::now()
//             .duration_since(UNIX_EPOCH)
//             .unwrap()
//             .as_millis();
//         println!("{}", time);
