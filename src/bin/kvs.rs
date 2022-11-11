use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;
use wasm_storage::store;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // env_logger::init();

    // 允许将要侦听的地址作为该程序的第一个参数进行传递，
    // 否则我们将在127.0.0.1：8080上为连接设置我们的TCP侦听器。
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:1234".to_string());

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await?;

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                socket
                    .write_all("&buf[0..n]".as_bytes())
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}
