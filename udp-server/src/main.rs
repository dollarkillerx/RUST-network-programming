use std::net::UdpSocket;
use std::{env,process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("please inout ./xxx 0.0.0.0:8081");
        process::exit(1);
    }

    let socket = UdpSocket::bind(args[1].clone()).expect("地址被占用!!!");
    loop {
        let mut buf = [0u8;1500];
        let (len,addr) = match socket.recv_from(&mut buf) {
            Ok((si,adr))=> (si,adr),
            Err(e)=>{
                eprintln!("err: {}",e);
                continue;
            },
        };

        println!("size: {}",len);
        buf.reverse(); // 反转字符串
        match socket.send_to(&buf,addr) {
            Ok(t) => {
                println!("Ok: {}",t);
                continue;
            },
            Err(e) => {
                eprintln!("err: {}",e);
                continue;
            }
        }
    }

}
