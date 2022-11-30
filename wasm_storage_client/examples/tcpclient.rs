use std::io::{Read, Write};
use wasmedge_wasi_socket::{Shutdown, TcpStream};

pub struct TcpClient {}

impl TcpClient {
    fn put(tcp_server: String, key: String, value: String) -> std::io::Result<()> {
        let mut stream = TcpStream::connect(tcp_server)?;
        stream.set_nonblocking(true)?;
        println!("sending put message");
        let cmd = format!("put {}={}.", key, value);
        stream.write(cmd.as_bytes())?;
        loop {
            let mut buf = [0; 128];
            match stream.read(&mut buf) {
                Ok(0) => {
                    println!("server closed connection");
                    break;
                }
                Ok(size) => {
                    let buf = &mut buf[..size];
                    println!("response: {}", String::from_utf8_lossy(buf));
                    // 直接关闭连接
                    stream.shutdown(Shutdown::Both)?;
                    break;
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        // 没读到数据就忽略继续等待回应，也不要执行其它操作
                        // println!("no data available, wait for 500ms");
                        // sleep(Duration::from_millis(500));
                        // println!("no data available");
                        // break;
                    } else {
                        return Err(e);
                    }
                }
            };
        }
        Ok(())
    }
    fn get(tcp_server: String, key: String) -> std::io::Result<()> {
        let mut stream = TcpStream::connect(tcp_server)?;
        stream.set_nonblocking(true)?;
        println!("sending get message");
        let cmd = format!("get {}.", key);
        stream.write(cmd.as_bytes())?;
        loop {
            let mut buf = [0; 128];
            match stream.read(&mut buf) {
                Ok(0) => {
                    println!("server closed connection");
                    break;
                }
                Ok(size) => {
                    let buf = &mut buf[..size];
                    println!("response: {}", String::from_utf8_lossy(buf));
                    // 直接关闭连接
                    stream.shutdown(Shutdown::Both)?;
                    break;
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        // 没读到数据就忽略继续等待回应，也不要执行其它操作
                        // println!("no data available, wait for 500ms");
                        // sleep(Duration::from_millis(500));
                        // println!("no data available");
                        // break;
                    } else {
                        return Err(e);
                    }
                }
            };
        }
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let start_time = chrono::Utc::now().timestamp_millis();
    println!("start_time : {} ms", start_time);
    // for i in 1..=5000 {
    //     println!("i : {}", i);
    //     TcpClient::put(
    //         "192.168.10.120:12000".to_string(),
    //         "key1".to_string(),
    //         "value1".to_string(),
    //     );
    //     TcpClient::get("192.168.10.120:12000".to_string(), "key1".to_string());
    // }
    let mut stream = TcpStream::connect("192.168.10.120:12000".to_string())?;
    stream.set_nonblocking(true)?;
    // let key = "key1";
    // let value = "value1";
    println!("sending put message");
    let cmd_put = String::from("put key1=value2222.");
    let cmd_get = String::from("get key1.");
    for i in 0..4 {
        println!("i: {}", i);
        if i % 2 != 1 {
            stream.write(cmd_put.as_bytes())?;
        } else {
            stream.write(cmd_get.as_bytes())?;
        }

        loop {
            let mut buf = [0; 128];
            match stream.read(&mut buf) {
                Ok(0) => {
                    println!("server closed connection");
                    break;
                }
                Ok(size) => {
                    let buf = &mut buf[..size];
                    println!("response: {}", String::from_utf8_lossy(buf));
                    // 直接关闭连接
                    // stream.shutdown(Shutdown::Both)?;
                    break;
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::WouldBlock {
                        // 没读到数据就忽略继续等待回应，也不要执行其它操作
                        // println!("no data available, wait for 500ms");
                        // sleep(Duration::from_millis(500));
                        // println!("no data available");
                        // break;
                    } else {
                        return Err(e);
                    }
                }
            };
        }
    }
    let end_time = chrono::Utc::now().timestamp_millis();
    println!("10000 querys takes : {} ms", end_time - start_time);
    // let port = std::env::var("PORT").unwrap_or("12000".to_string());
    // println!("connect to 192.168.10.120:{}", port);
    // let mut stream = TcpStream::connect(format!("192.168.10.120:{}", port))?;
    // stream.set_nonblocking(true)?;
    // println!("sending put message");
    // stream.write(b"put key1=value1")?;

    // loop {
    //     let mut buf = [0; 128];
    //     match stream.read(&mut buf) {
    //         Ok(0) => {
    //             println!("server closed connection");
    //             break;
    //         }
    //         Ok(size) => {
    //             let buf = &mut buf[..size];
    //             println!("get response: {}", String::from_utf8_lossy(buf));
    //             stream.shutdown(Shutdown::Both)?;
    //             break;
    //         }
    //         Err(e) => {
    //             if e.kind() == std::io::ErrorKind::WouldBlock {
    //                 println!("no data available, wait for 500ms");
    //                 sleep(Duration::from_millis(500));
    //             } else {
    //                 return Err(e);
    //             }
    //         }
    //     };
    // }
    Ok(())
}
