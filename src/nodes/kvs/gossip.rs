use self::gossip_rpc::gossip_server::Gossip;
use super::KvsNode;
use crate::store::Store;
use gossip_rpc::gossip_client::GossipClient;
use gossip_rpc::gossip_server::GossipServer;
use gossip_rpc::{AppendEntriesInGossipArgs, AppendEntriesInGossipReply};
use lazy_static::lazy_static;
use log::debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod gossip_rpc {
    tonic::include_proto!("gossip_proto");
}

// 创建全局变量：动态数组GOSSIP_QUEUE，记录待同步的key
lazy_static! {
    pub static ref GOSSIP_QUEUE: Mutex<Vec<String>> = Mutex::new(vec![]);
}
// 创建全局变量：GOSSIP_KEY_COUNT，记录待同步的key的数量
pub static GOSSIP_KEY_COUNT: AtomicUsize = AtomicUsize::new(0);
pub struct GossipEntity {}

// 实现 Gossip特征：append_entries_in_gossip server端的处理
#[tonic::async_trait]
impl Gossip for GossipEntity {
    // append_entries_in_gossip 服务端处理函数
    async fn append_entries_in_gossip(
        &self,
        request: Request<AppendEntriesInGossipArgs>,
    ) -> Result<Response<AppendEntriesInGossipReply>, Status> {
        let key = request.get_ref().key.clone();
        let value = request.get_ref().value.clone();
        debug!(
            "KvsNode Got a RPC Request From: {:?}: key={}, value={}",
            request.remote_addr().unwrap(),
            key,
            value
        );
        // RPC Server创建时Rust的所有权问题，创建临时数据库gossip
        // let mut gossip_db = Store::init(String::from("db/gossip_db"));
        // gossip_db.put(key.clone(), value.clone()).unwrap();
        GOSSIP_KEY_COUNT.fetch_add(1, Ordering::Relaxed);
        GOSSIP_QUEUE.lock().unwrap().push(key);
        debug!(
            "KvsNode Got a RPC append_entries_in_gossip, Update GOSSIP_KEY_COUNT: {:?}",
            GOSSIP_KEY_COUNT,
        );
        let reply = gossip_rpc::AppendEntriesInGossipReply { success: true };
        Ok(Response::new(reply))
    }
}

impl KvsNode {
    // kvs作为客户端向其他kvs发送追加日志请求
    pub async fn send_append_entries_in_gossip(
        &self,
        key_arg: String,
        value_arg: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // 让所有的KvsNode同步操作
        for member in self.get_membership() {
            // RPC Server地址
            let dst = "http://".to_string() + member.as_str();
            let mut client = GossipClient::connect(dst).await.unwrap();
            let request = tonic::Request::new(AppendEntriesInGossipArgs {
                key: key_arg.clone(),
                value: value_arg.clone(),
            });
            let response = client.append_entries_in_gossip(request).await.unwrap();
            if response.get_ref().success == true {}
        }
        Ok(())
    }
}

// 启动RPC Server
pub async fn make_gossip_server(
    gossip_entity: GossipEntity,
    addr: String,
) -> Result<(), tonic::transport::Error> {
    let to_kvs_addr = addr.parse().unwrap();
    Server::builder()
        .add_service(GossipServer::new(gossip_entity))
        .serve(to_kvs_addr)
        .await
}
