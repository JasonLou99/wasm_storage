use rocksdb::DB;

pub struct Store {
    db: DB,
}

impl Store {
    // 初始化底层存储
    pub fn init(path: String) -> Store {
        Store {
            db: DB::open_default(path).unwrap(),
        }
    }

    // 写值
    pub fn put(&mut self, key: String, value: String) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.db.put(key, value)?;
        Ok(res)
    }

    // 读值
    pub fn get(&mut self, key: String) -> Result<String, rocksdb::Error> {
        match self.db.get(key) {
            Ok(Some(value)) => return Ok(String::from_utf8(value).unwrap()),
            Ok(None) => {
                println!("value not found");
                return Ok(String::from(""));
            }
            Err(e) => {
                println!("operational problem encountered: {}", e);
                return Err(e);
            }
        }
    }
}

#[cfg(test)]
mod db_tests {
    use rocksdb::{Options, DB};
    #[test]
    fn get() {
        // NB: db is automatically closed at end of lifetime
        let path = "demo";
        {
            let db = DB::open_default(path).unwrap();
            db.put(b"my key", b"my value").unwrap();
            match db.get(b"my key") {
                Ok(Some(value)) => {
                    println!(
                        "!!!!!!!!!!!!!!!!!!!!!!retrieved value {}",
                        String::from_utf8(value).unwrap()
                    )
                }
                Ok(None) => println!("value not found"),
                Err(e) => println!("operational problem encountered: {}", e),
            }
            db.delete(b"my key").unwrap();
        }
        let _ = DB::destroy(&Options::default(), path);
    }
}
