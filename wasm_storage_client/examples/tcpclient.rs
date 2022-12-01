use std::io::{Read, Write};
use wasmedge_wasi_socket::{Shutdown, TcpStream};

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
// 在单个Tcp连接下，发送总计times次请求，并且put比例为ratio
fn put_get_multitimes(times: i32, ratio: f32) -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.10.120:12000".to_string())?;
    let put_times = (ratio * 10.0) as i32;
    let get_times = ((1.0 - ratio) * 10.0) as i32;
    for i in 0..(times / 10) {
        for m in 0..put_times {
            let cmd_put = format!("put key{}=value{}.", m + 10 * i, m + 10 * i);
            stream.write(cmd_put.as_bytes())?;
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
                        break;
                    }
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::WouldBlock {
                        } else {
                            return Err(e);
                        }
                    }
                };
            }
        }
        for _ in 0..get_times {
            let cmd_get = format!("get key{}.", 10 * i);
            stream.write(cmd_get.as_bytes())?;
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
                        break;
                    }
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::WouldBlock {
                        } else {
                            return Err(e);
                        }
                    }
                };
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let start_time = chrono::Utc::now().timestamp_millis();
    let res = put_get_multitimes(10000, 0.5);
    let end_time = chrono::Utc::now().timestamp_millis();
    println!(
        "start_time: {}, end_time:{}, 10000 querys takes : {} ms",
        start_time,
        end_time,
        end_time - start_time
    );
    res

    /* let rt1 = Runtime::new().unwrap();
    let start_time = chrono::Utc::now().timestamp_millis();
    println!("start_time : {} ms", start_time);
    // multi clients
    // for _ in 0..10 {
    //     tokio::spawn(async move {
    //         println!("{}", 10);
    //         // let mut stream = TcpStream::connect("192.168.10.120:12000".to_string()).unwrap();
    //         // stream.set_nonblocking(true).unwrap();
    //         // put_get_multitimes(10, 0.5);
    //     });
    // }

    /*
    let cmd_put = String::from("put key1=value2222.");
    let cmd_get = String::from("get key1.");

    for client in 0..2 {
        for i in 0..2 {
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
    } */
    let end_time = chrono::Utc::now().timestamp_millis();
    println!("10000 querys takes : {} ms", end_time - start_time);
    Ok(()) */
}
