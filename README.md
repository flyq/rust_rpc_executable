# rust_rpc_executable
call the executable program use rpc

## how to use
1. in server, install the cita-cli
2. clone this repo:  `https://github.com/flyq/rust_rpc_executable.git`
3. in terminal 1: run: `cd rust_rpc_executable && cargo run`
4. in terminal 2, run rpc call:
```shell
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "say_hello", "params": [], "id":123 }' 127.0.0.1:3030
```
logout:  
terminal 1: `Cargo.lock  Cargo.toml  README.md  src  target   `  
terminal 2: `{"jsonrpc":"2.0","result":"hello, ls","id":123}`

```shell
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "test", "params": [], "id":123 }' 127.0.0.1:3030
```

logout:  
terminal 1: `status: exit code: 0 `  
terminal 2: `{"jsonrpc":"2.0","result":"test, sh","id":123}`   
and the book dir is maked already.

```shell
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "param_test", "params": [1024], "id":123 }' 127.0.0.1:3030
```
terminal 1: ``    
terminal 2: `{"jsonrpc":"2.0","result":"world: 1024","id":123}`

```shell
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "cita-cli", "params": {"subcom1": "rpc", "subcom2": "sendRawTransaction", "code": "0x36555b8500000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000017094f45842000000000000000000000000000000000000000000000000000000000000000b434f4445204953204c4157000000000000000000000000000000000000000000", "private-key": "0xcba2a9c5b037ba844e695d019ef6c596e9af150d8ad03567d42b9d977e0e41c1", "address": "0x3b6c6C8022c858bA316122A873A4EF974754D247"}, "id":123 }' 127.0.0.1:303
```
terminal 2: `curl: (7) Failed to connect to 127.0.0.1 port 303: Connection refused`

```shell
curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "cita-cli", "params": {"subcom1": "rpc", "subcom2": "sendRawTransaction", "code": "0x36555b8500000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000017094f45842000000000000000000000000000000000000000000000000000000000000000b434f4445204953204c4157000000000000000000000000000000000000000000", "private-key": "0xcba2a9c5b037ba844e695d019ef6c596e9af150d8ad03567d42b9d977e0e41c1", "address": "0x3b6c6C8022c858bA316122A873A4EF974754D247"}, "id":123 }' 127.0.0.1:3030
```
terminal 1: `status: exit code: 1 `   
terminal 2: `{"jsonrpc":"2.0","result":"cita-cli run success","id":123}`   




### rawTX
```shell
cita-cli rpc sendRawTransaction \
    --code 0x36555b8500000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000017094f45842000000000000000000000000000000000000000000000000000000000000000b434f4445204953204c4157000000000000000000000000000000000000000000 \
    --private-key 0xcba2a9c5b037ba844e695d019ef6c596e9af150d8ad03567d42b9d977e0e41c1 \
    --address 0x3b6c6C8022c858bA316122A873A4EF974754D247 \
    --url http://127.0.0.1:1337

```
