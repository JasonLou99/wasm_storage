use crate::store::Store;

// 找同级的gossip.rs 或者 gossip/mod.rs
pub mod gossip;

#[allow(dead_code)]
pub struct KvsNode {
    // 本节点地址
    node_id: String,
    // 集群成员信息
    membership: Vec<String>,
    // 底层存储
    store: Store,
}

impl KvsNode {
    pub fn init(node_id_arg: String, membership_arg: Vec<String>) -> KvsNode {
        Self::new(node_id_arg, membership_arg)
    }

    fn new(node_id_arg: String, membership_arg: Vec<String>) -> KvsNode {
        Self {
            node_id: node_id_arg.clone(),
            membership: membership_arg,
            store: Store::init(String::from("db/kv_db")),
        }
    }

    pub fn get_node_id(&self) -> String {
        self.node_id.clone()
    }

    pub fn get_membership(&self) -> Vec<String> {
        self.membership.clone()
    }

    pub async fn put(
        &mut self,
        key_arg: String,
        value_arg: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.store.put(key_arg, value_arg)
    }

    pub async fn get(&mut self, key_arg: String) -> Result<String, rocksdb::Error> {
        self.store.get(key_arg)
    }
}
