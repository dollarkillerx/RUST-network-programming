use std::thread;
use std::env;
use std::net::{TcpListener,TcpStream};
use std::io::{BufReader, BufRead, Write};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("please ./xxx 0.0.0.0:8081");
        std::process::exit(1);
    }

    let listen = TcpListener::bind(args[1].clone()).unwrap();
    for conn in listen.incoming() {
        thread::spawn(||{
            conn_server(conn.unwrap());
        });
    }
}

fn conn_server(conn: TcpStream) {
    println!("addr: {}",conn.peer_addr().unwrap());
    let mut data = Vec::new();
    let mut buffer = BufReader::new(conn);
    loop {
        data.clear();

        let bytes_read =  buffer.read_until(b'\n',&mut data).unwrap();
        if bytes_read == 0 {
            break;
        }

        let mut input: User = serde_json::from_slice(&data).unwrap();
        println!("User: {:?}",input);

        input.name.push_str("Acid");
        let c = serde_json::to_string(&input).unwrap();

        buffer.get_mut().write_all(c.as_bytes()).unwrap();
        buffer.get_mut().write_all(b"\n").unwrap();
        buffer.get_mut().flush().unwrap();
    }
}
