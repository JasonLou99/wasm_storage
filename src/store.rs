//  基于HashMap的键值存储
use chrono::Local;
use std::collections::{hash_map, HashMap};

pub struct TimestampValue {
    value: Vec<u8>,
    timestamp: i64,
}

impl TimestampValue {
    pub fn new(v: Vec<u8>) -> TimestampValue {
        let ts = Local::now().timestamp();
        TimestampValue {
            value: v,
            timestamp: ts,
        }
    }

    pub fn merge(&mut self, other: &TimestampValue) {
        Local::now().timestamp();
        if self.timestamp < other.timestamp {
            self.timestamp = other.timestamp;
            self.value = other.value.clone();
        }
    }
}

pub struct TsValueStore {
    db: HashMap<Vec<u8>, TimestampValue>,
}

impl TsValueStore {
    pub fn new() -> TsValueStore {
        TsValueStore { db: HashMap::new() }
    }

    pub fn put(
        &mut self,
        key: Vec<u8>,
        value: TimestampValue,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

    pub fn get(&self, key: &Vec<u8>) -> Option<Vec<u8>> {
        match self.db.get(key) {
            Some(ts) => Some(ts.value.clone()),
            None => None,
        }
    }
}

// 获得当前时间戳
// Local::now().timestamp();

#[cfg(test)]
mod store_tests {
    use std::collections::HashMap;

    use super::{TimestampValue, TsValueStore};

    #[test]
    fn get() {
        let map1: HashMap<_, _> = [
            (b"a".to_vec(), TimestampValue::new(b"10".to_vec())),
            (b"b".to_vec(), TimestampValue::new(b"20".to_vec())),
        ]
        .into_iter()
        .collect();

        let mut store = TsValueStore::new();
        store.db = map1;

        assert_eq!(store.get(&b"a".to_vec()), Some(b"10".to_vec()));
        assert_eq!(store.get(&b"ccc".to_vec()), None);
    }
}
