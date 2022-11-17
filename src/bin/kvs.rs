use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tonic::{transport::Server, Request, Response, Status};

use std::env;
use std::error::Error;
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
    println!("1");
    let to_kvs_addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();
    tokio::spawn(async move {
        Server::builder()
            .add_service(GreeterServer::new(greeter))
            .serve(to_kvs_addr)
            .await
            .unwrap();
    });
    // env_logger::init();
    println!("2");
    // 允许将要侦听的地址作为该程序的第一个参数进行传递，
    // 否则我们将在127.0.0.1:12000上为连接设置我们的TCP侦听器。
    let to_client_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:12000".to_string());

    // 创建一个TCP侦听器，它将侦听传入的连接。
    // 此TCP侦听器绑定到我们上面确定的地址，并且必须与事件循环相关联。
    let mut listener = TcpListener::bind(&to_client_addr).await?;
    println!("Listening on: {}", to_client_addr);

    loop {
        println!("test1");
        // 异步等待inbound套接字
        let (mut socket, _) = listener.accept().await?;

        // 我们希望所有客户同时取得进展，而不是在一个客户完成另一个客户后阻止另一个客户。
        // 为此，我们使用`tokio：：spawn`函数在后台执行工作。
        // 本质上，我们在这里执行一个并发运行的新任务，这将允许并发处理我们的所有客户端。
        tokio::spawn(async move {
            println!("test2");
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

                println!("{}", String::from_utf8_lossy(&buf));
                socket
                    .write_all("&buf[0..n]".as_bytes())
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}
