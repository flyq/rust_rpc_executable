use jsonrpc_http_server::*;
use jsonrpc_http_server::jsonrpc_core::*;
use std::process::Command;

fn main() {
        let mut io = IoHandler::default();
        io.add_method("say_hello", |_| {
                let mut list_dir = Command::new("ls");
                list_dir.status().expect("process failed to execute");

                Ok(Value::String("hello, ls".into()))
        });

        let server = ServerBuilder::new(io)
                .cors(DomainsValidation::AllowOnly(vec![AccessControlAllowOrigin::Null]))
                .start_http(&"127.0.0.1:3030".parse().unwrap())
                .expect("Unable to start RPC server");

        server.wait();
}
