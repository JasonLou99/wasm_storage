use crate::store::TsValueStore;

// 找同级的gossip.rs 或者 gossip/mod.rs
mod gossip;

pub struct KvsNode {
    // 本节点地址
    node_id: String,
    // 集群成员信息
    membership: Vec<String>,
    // 底层存储
    kvs: TsValueStore,
}

impl KvsNode {
    pub async fn init(node_id_arg: String, membership_arg: Vec<String>) -> KvsNode {
        Self::new(node_id_arg, membership_arg)
    }

    fn new(node_id_arg: String, membership_arg: Vec<String>) -> KvsNode {
        Self {
            node_id: node_id_arg.clone(),
            membership: membership_arg,
            kvs: TsValueStore::new(),
        }
    }
}
