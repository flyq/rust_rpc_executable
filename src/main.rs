use jsonrpc_http_server::jsonrpc_core::*;
use jsonrpc_http_server::*;
use std::process::Command;
use std::collections::HashMap;

fn main() {
    let mut io = IoHandler::default();
    io.add_method("say_hello", |_| {
        let mut list_dir = Command::new("ls");
        list_dir.status().expect("process failed to execute");

        Ok(Value::String("hello, ls".into()))
    });

    io.add_method("test", |_| {
        let output = Command::new("mkdir")
            .arg("book")
            .output()
            .expect("failed to execute process");

        println!("status: {} ", output.status);

        Ok(Value::String("test, sh".into()))
    });

    io.add_method("param_test", |params: Params| {
        match params.parse::<(u64,)>() {
            Ok((num,)) => Ok(Value::String(format!("world: {}", num))),
            _ => Ok(Value::String("world".into())),
        }
    });

    io.add_method("cita-cli", |params: Params| {
        match params.parse::<HashMap<String, String>>() {
            Ok(map) => {
                let cita = Command::new("cita-cli")
                    .arg(map.get("subcom1").unwrap())
                    .arg(map.get("subcom2").unwrap())
                    .arg("--code")
                    .arg(map.get("code").unwrap())
                    .arg("--private-key")
                    .arg(map.get("private-key").unwrap())
                    .arg("--address")
                    .arg(map.get("address").unwrap())
                    .arg("--url http://127.0.0.1:1337")
                    .output()
                    .expect("failed to execute process");

                println!("status: {} ", cita.status);

                Ok(Value::String("cita-cli run success".into()))
            },
            _ => Ok(Value::String("world".into())),
        }
    });

    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Null,
        ]))
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .expect("Unable to start RPC server");

    server.wait();
}
