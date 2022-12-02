#!/bin/bash
start_sec=`date '+%s'`
for (( i = 1; i <= 1; i++ ))
do
    {
        wasmedge ./wasm_storage_client/target/wasm32-wasi/debug/examples/tcpclient.wasm
    } > ./client$i.log 2>&1 &
done

while true   # 无限循环
# 检测正在运行的tcpclient数量
count=`ps -aux |grep "tcpclient.wasm" |grep -v "grep" |wc -l`
# echo $count
do 
    # 没有在运行的tcpclient，则终止脚本，并统计时间
    if [[ $count -eq 0 ]]
    then
        end_sec=`date '+%s'`
        let time=($end_sec-$start_sec)
        echo "耗时：$time s"
        break
    fi
done