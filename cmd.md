编译kvs：`cargo build --bin kvs`
运行kvs：`RUST_LOG=kvs ./target/debug/kvs 192.168.10.120:11000 192.168.10.121 192.168.10.122 > ~/wasm_storage/kvs.log 2>&1 &`