# rust_rpc_executable
call the executable program use rpc

## how to use
1. in server, install the cita-cli
2. clone this repo:  `https://github.com/flyq/rust_rpc_executable.git`
3. `cd rust_rpc_executable && cargo run`
4. in another terminal: `curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "say_hello", "params": [], "id":123 }' 127.0.0.1:3030`
5. test 2: `curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "cita-cli", "params": {"subcom1": "rpc", "subcom2": "sendRawTransaction", "code": "0x36555b8500000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000017094f45842000000000000000000000000000000000000000000000000000000000000000b434f4445204953204c4157000000000000000000000000000000000000000000", "private-key": "0xcba2a9c5b037ba844e695d019ef6c596e9af150d8ad03567d42b9d977e0e41c1", "address": "0x3b6c6C8022c858bA316122A873A4EF974754D247"}, "id":123 }' 127.0.0.1:3030`




cita-cli rpc sendRawTransaction \
    --code 0x36555b8500000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000017094f45842000000000000000000000000000000000000000000000000000000000000000b434f4445204953204c4157000000000000000000000000000000000000000000 \
    --private-key 0xcba2a9c5b037ba844e695d019ef6c596e9af150d8ad03567d42b9d977e0e41c1 \
    --address 0x3b6c6C8022c858bA316122A873A4EF974754D247 \
    --url http://127.0.0.1:1337

curl -X POST -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "cita-cli", "params": [56], "id":123 }' 127.0.0.1:3030
