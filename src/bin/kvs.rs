use log::{debug, error, info, log_enabled, Level};
use std::env;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use wasm_storage::nodes::kvs::gossip;
use wasm_storage::nodes::kvs::KvsNode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // 初始化KvsNode
    let kvs_node = KvsNode::init(
        String::from("192.168.10.120:11000"),
        vec![
            String::from("192.168.10.120:11000"),
            String::from("192.168.10.121:11000"),
        ],
    );
    // 开启RPC Server
    gossip::make_gossip_server(kvs_node, String::from("192.168.10.120:11000"))
        .await
        .unwrap();

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
                    KvsNode::append_entries_to_others().await.unwrap();
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
