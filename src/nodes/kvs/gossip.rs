use self::gossip_rpc::gossip_server::Gossip;
use super::KvsNode;
use gossip_rpc::gossip_client::GossipClient;
use gossip_rpc::gossip_server::GossipServer;
use gossip_rpc::{AppendEntriesInGossipArgs, AppendEntriesInGossipReply};
use log::debug;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod gossip_rpc {
    tonic::include_proto!("gossip_proto");
}

pub struct GossipEntity {}

// 实现 Gossip特征：append_entries_in_gossip server端的处理
#[tonic::async_trait]
impl Gossip for GossipEntity {
    // append_entries_in_gossip 服务端处理函数
    async fn append_entries_in_gossip(
        &self,
        request: Request<AppendEntriesInGossipArgs>,
    ) -> Result<Response<AppendEntriesInGossipReply>, Status> {
        println!("Got a rpc request: {:?}", request);
        let reply = gossip_rpc::AppendEntriesInGossipReply { success: true };
        Ok(Response::new(reply))
    }
}

impl KvsNode {
    // kvs作为客户端向其他kvs发送追加日志请求
    pub async fn send_append_entries_in_gossip(&self) -> Result<(), Box<dyn std::error::Error>> {
        // for member in self.get_membership() {
        //     let mut client = GossipClient::connect(member).await.unwrap();
        //     let request = tonic::Request::new(AppendEntriesInGossipArgs {
        //         log: "gossip say hello".into(),
        //     });
        //     let response = client.append_entries_in_gossip(request).await.unwrap();
        // }
        let mut client = GossipClient::connect("http://192.168.10.120:11000")
            .await
            .unwrap();
        let request = tonic::Request::new(AppendEntriesInGossipArgs {
            log: "gossip say hello".into(),
        });
        let response = client.append_entries_in_gossip(request).await.unwrap();
        debug!("append_entries_in_gossip RPC response: {:?}", response);
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
