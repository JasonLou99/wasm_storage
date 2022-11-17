use super::KvsNode;
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

impl KvsNode {
    pub async fn gossip_say_hello() -> Result<(), Box<dyn std::error::Error>> {
        println!("gossip_say_hello");
        let mut client = GreeterClient::connect("http://192.168.10.121:50051").await?;
        let request = tonic::Request::new(HelloRequest {
            name: "gossip say hello".into(),
        });

        let response = client.say_hello(request).await?;

        println!("gossip_say_hello RESPONSE={:?}", response);

        Ok(())
    }
}
