// use futures::executor::block_on;
use log::{debug, info};
use std::env;
use std::error::Error;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use wasm_storage::nodes::kvs::gossip::{self, GossipEntity};
use wasm_storage::nodes::kvs::KvsNode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "kvs,wasm_storage"),
    )
    .init();

    let args: Vec<String> = env::args().collect();

    let node_id = &args[1];
    let tcp_server_addr = &args[2];
    let membership = &args[3..];

    // 初始化KvsNode
    let kvs_node = KvsNode::init(node_id.to_string(), membership.to_vec());
    info!("kvsnode init success!");
    info!("kvsnode node_id: {}", kvs_node.get_node_id());
    info!("kvsnode membership: {:?}", kvs_node.get_membership());
    // 开启RPC Server
    let rpc_server_addr = kvs_node.get_node_id();
    let gossip_entity = GossipEntity {};
    tokio::spawn(async move {
        gossip::make_gossip_server(gossip_entity, String::from(rpc_server_addr))
            .await
            .unwrap();
    });
    debug!("GOSSIP RPC Server init success!");
    debug!("RPC for Kvs Listening on: {}", kvs_node.get_node_id());

    // 默认会以12000端口为TCP端口
    let to_client_addr = tcp_server_addr.to_string();

    // 创建一个TCP侦听器，它将侦听传入的连接。
    let listener = TcpListener::bind(&to_client_addr).await?;
    debug!("TCP for Client Listening on: {}", to_client_addr);

    let arc_kvs_node = Arc::new(Mutex::new(kvs_node));

    loop {
        // 因为TCP需要异步并行接收多个连接，所以下面使用tokio::spawn，但是会move kvs_node的所有权
        // 所以这里使用Arc clone的方式，每次循环clone一个所有者，实现单个kvs_node多个所有者并行处理TCP连接
        let arc_kvs_node_clone = Arc::clone(&arc_kvs_node);
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
                    info!("Client Operation: put");
                    let all_msg = &client_op[0..client_op.len()];
                    let put_key_value = &all_msg[4..client_op.len()];
                    let (put_key, put_value) = put_key_value.rsplit_once('=').unwrap();
                    let (put_value, _) = put_value.rsplit_once(".").unwrap();
                    // 本节点PUT操作执行完毕之后再执行Gossip同步

                    arc_kvs_node_clone
                        .lock()
                        .await
                        .put(put_key.to_string(), put_value.to_string())
                        .await
                        .unwrap();

                    debug!("KvsNode put {} {}", put_key, put_value);
                    socket
                        .write_all("put success".as_bytes())
                        .await
                        .expect("failed to write data to socket");
                    // block_on(socket.shutdown());
                    // 异步gossip传播同步put操作
                    debug!("kvs send RPC request: send_append_entries_in_gossip");
                    arc_kvs_node_clone
                        .lock()
                        .await
                        .send_append_entries_in_gossip(put_key.to_string(), put_value.to_string())
                        .await
                        .unwrap();
                } else if client_op.starts_with("get") {
                    info!("Client Operation: get");
                    let all_msg = &client_op[0..client_op.len()];
                    let get_key_value = &all_msg[4..client_op.len()];
                    let (get_key_temp, _) = get_key_value.rsplit_once('.').unwrap();
                    let (get_key, _) = get_key_temp.rsplit_once('.').unwrap();
                    // println!("打开gossip db");
                    // let mut temp_store = Store::init(String::from("db/gossip_db"));
                    // let get_value_from_gossipdb = temp_store.get(get_key.to_string()).unwrap();
                    let get_value = arc_kvs_node_clone
                        .lock()
                        .await
                        .get(get_key.to_string())
                        .await
                        .unwrap();
                    // println!("get_value: {}", get_value);
                    debug!("KvsNode get {}={} From kv_db", get_key, get_value);
                    // debug!(
                    //     "KvsNode get {}={} From gossip_db",
                    //     get_key, get_value_from_gossipdb
                    // );
                    // debug!("GOSSIP_KEY_COUNT: {:?}", GOSSIP_KEY_COUNT);
                    // debug!("GOSSIP_QUEUE:{:?}", GOSSIP_QUEUE.lock().unwrap());
                    let tcp_resp = format!("get {}={}", get_key, get_value);
                    // println!("{}", tcp_resp);
                    socket
                        .write_all(tcp_resp.as_bytes())
                        .await
                        .expect("failed to write data to socket");
                    // socket.shutdown().await.unwrap();
                } else {
                    info!("client operation is error");
                    socket
                        .write_all("your operation is wrong, only put or get".as_bytes())
                        .await
                        .expect("failed to write data to socket");
                }
            }
        });
    }
}
