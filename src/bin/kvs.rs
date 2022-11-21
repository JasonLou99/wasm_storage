use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use std::env;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tonic::{transport::Server, Request, Response, Status};
use wasm_storage::nodes::kvs::KvsNode;
// use wasm_storage::store;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // 接收以HelloRequest为类型的请求
    ) -> Result<Response<HelloReply>, Status> {
        // 返回以HelloReply为类型的示例作为响应
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(), // 由于gRPC请求和响应中的字段都是私有的，所以需要使用 .into_inner()
        };

        Ok(Response::new(reply)) // 发回格式化的问候语
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let to_kvs_addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();
    // 开启RPC Server，处理kvs调用
    tokio::spawn(async move {
        Server::builder()
            .add_service(GreeterServer::new(greeter))
            .serve(to_kvs_addr)
            .await
            .unwrap();
    });

    // 第一个参数是TCP监听端口，默认会以12000端口为TCP端口
    let to_client_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "192.168.10.120:12000".to_string());

    // 创建一个TCP侦听器，它将侦听传入的连接。
    let mut listener = TcpListener::bind(&to_client_addr).await?;
    println!("Listening on: {}", to_client_addr);

    loop {
        // 异步等待套接字
        let (mut socket, _) = listener.accept().await?;

        // 使用绿色线程异步的处理多个任务
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            // 在循环中，从套接字读取数据并将数据写回。
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }
                let client_op = String::from_utf8_lossy(&buf);
                if client_op.starts_with("put") {
                    println!("client_op: put");
                    KvsNode::gossip_say_hello().await.unwrap();
                } else if client_op.starts_with("get") {
                    println!("client_op: get");
                } else {
                    println!("client_op: others");
                }
                socket
                    .write_all("&buf[0..n]".as_bytes())
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}
