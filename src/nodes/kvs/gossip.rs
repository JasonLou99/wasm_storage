use self::gossip_rpc::gossip_server::Gossip;

use super::KvsNode;
use gossip_rpc::gossip_client::GossipClient;
use gossip_rpc::gossip_server::GossipServer;
use gossip_rpc::{AppendEntriesInGossipArgs, AppendEntriesInGossipReply};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod gossip_rpc {
    tonic::include_proto!("gossip_proto");
}

// 实现 Gossip特征，也就是RPC中的Gossip Service内的方法
#[tonic::async_trait]
impl Gossip for KvsNode {
    async fn append_entries_in_gossip(
        &self,
        request: Request<AppendEntriesInGossipArgs>,
    ) -> Result<Response<AppendEntriesInGossipReply>, Status> {
        println!("Got a request: {:?}", request);
        let reply = gossip_rpc::AppendEntriesInGossipReply { success: true };
        Ok(Response::new(reply))
    }
}

impl KvsNode {
    // kvs作为客户端向其他kvs发送追加日志请求
    pub async fn append_entries_to_others() -> Result<(), Box<dyn std::error::Error>> {
        println!("gossip_say_hello");

        let mut client = GossipClient::connect("http://192.168.10.121:12000").await?;
        let request = tonic::Request::new(AppendEntriesInGossipArgs {
            log: "gossip say hello".into(),
        });

        let response = client.append_entries_in_gossip(request).await?;

        println!("gossip_say_hello RESPONSE={:?}", response);

        Ok(())
    }
}

// 启动RPC Server
pub async fn make_gossip_server(
    kvs: KvsNode,
    addr: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let to_kvs_addr = addr.parse()?;
    Server::builder()
        .add_service(GossipServer::new(kvs))
        .serve(to_kvs_addr)
        .await
        .unwrap();
    println!("Start RPC Server");
    Ok(())
}
