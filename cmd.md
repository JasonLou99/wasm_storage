编译kvs：`cargo build --bin kvs`

运行kvs：
`./target/debug/kvs <kvs rpc server> <kvs tcp server> <kvs member rpc server...> > ~/wasm_storage/kvs.log 2>&1 &`

示例：
`./target/debug/kvs 192.168.10.120:11000 192.168.10.120:12000 192.168.10.121:11000 192.168.10.122:11000 > ~/wasm_storage/kvs.log 2>&1 &`
