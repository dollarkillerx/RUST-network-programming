use std::io::{BufReader, Write, stdin, BufRead};
use std::net::TcpStream;
use std::{env,process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("please input ./xxx 0.0.0.0:8081");
        process::exit(1);
    }

    let mut conn = TcpStream::connect(args[1].clone()).expect("地址被占用");
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("你输入啥子蛤???");

        conn.write(input.as_bytes()).expect("没有发送过去");

        let mut reader = BufReader::new(&conn);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_until(b'\n',&mut buffer).expect("读取失败");

        println!("read msg: {}",String::from_utf8_lossy(&buffer[..]));
        println!();
    }
}
