/* 这个程序有问题，不能多线程调用wasm程序发送tcp请求 */

use std::path::Path;
use std::sync::Arc;
use wasmedge_sys::*;

#[tokio::main]
async fn main() {
    let mut config = Config::create().unwrap();
    config.wasi(true);

    let mut vm = Vm::create(Some(config), None).unwrap();

    // // get default wasi module
    // let mut wasi_module = vm.wasi_module_mut().unwrap();
    // // init the default wasi module
    // wasi_module.init_wasi(Some(vec![]), Some(vec![]), Some(vec![]));

    // example_wasmedge_sys
    let wasm_path =
        Path::new("/home/jason/wasm_storage/wasm_storage_client/target/wasm32-wasi/debug/examples/tcpclient.wasm");
    let _ = vm.load_wasm_from_file(wasm_path);
    let _ = vm.validate();
    let _ = vm.instantiate();

    let vm = Arc::new(vm);

    let mut v = Vec::new();

    for i in 0..10 {
        let l = vm.clone();
        let handle = tokio::spawn(async move {
            let _ = l.run_function("put_get_multitimes", vec![]);
            // let _ = l.run_function("t", vec![]);
        });
        println!("client{} run", i);
        v.push(handle);
    }

    futures::future::join_all(v).await;
}
