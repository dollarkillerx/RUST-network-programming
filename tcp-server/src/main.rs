use std::net::TcpListener;
use std::{process,thread};
use std::env;
use tcp_server::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please input ./xxx 0.0.0.0:8081");
        process::exit(1);
    }
    let listen = TcpListener::bind(args[1].clone()).expect("网络地址被占用");
    println!("Listen addr: {}",listen.local_addr().unwrap());
    for conn in listen.incoming() {
        match conn {
            Ok(conn) => {
                thread::spawn(move ||{
                    // server::server(conn).unwrap();
                    server::server(conn).unwrap_or_else(|err| eprintln!("ex server err: {:?}",err));
                });
            },
            Err(e) => {
                eprintln!("listen accept err: {:?}",e);
            },
        }
    }

    Ok(())
}

