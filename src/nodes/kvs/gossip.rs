use super::KvsNode;
use gossip::gossip_client::GossipClient;
use gossip::{AppendEntriesInGossipArgs, AppendEntriesInGossipReply};

pub mod gossip {
    tonic::include_proto!("gossip_proto");
}

impl KvsNode {
    pub async fn append_entries_in_gossip() -> Result<(), Box<dyn std::error::Error>> {
        println!("gossip_say_hello");

        let mut client = GossipClient::connect("http://192.168.10.121:50051").await?;
        let request = tonic::Request::new(AppendEntriesInGossipArgs {
            log: "gossip say hello".into(),
        });

        let response = client.append_entries_in_gossip(request).await?;

        println!("gossip_say_hello RESPONSE={:?}", response);

        Ok(())
    }
}
