use std::net::UdpSocket;
use std::{env,process};
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("please inout ./xxx 0.0.0.0:8081");
        process::exit(1);
    }

    // let socket = UdpSocket::bind(args[1].clone()).expect("bind IP 1");
    // socket.connect(args[1].clone()).expect("bind IP 2");

    // let socket = UdpSocket::bind("0.0.0.0:8081").expect("bind IP 1");
    let socket = UdpSocket::b;
    socket.connect(args[1].clone()).expect("bind IP 2");

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("e: {}",e);
                continue;
            },
        };

        match socket.send(input.as_bytes()) {
            Ok(o) => {
                println!("success: {}",o);
            },
            Err(e) => {
                eprintln!("e: {}",e);
                continue;
            },
        }

        let mut buffer = [0u8;1500];
        match socket.recv_from(&mut buffer) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("e: {}",e);
                continue;
            },
        };

        println!("read msg: {}",String::from_utf8_lossy(&buffer));
    }
}
