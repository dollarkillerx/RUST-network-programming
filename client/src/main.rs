use std::env;
use std::net::{TcpStream};
use std::io::{BufReader, Write, BufRead};
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

    let wang = User{
        name: String::from("Wang"),
        age: 18,
    };

    let dec = serde_json::to_string(&wang).unwrap();

    let mut conn = TcpStream::connect(args[1].clone()).unwrap();
    conn.write_all(dec.as_bytes()).unwrap();
    conn.write_all(b"\n").unwrap();
    conn.flush().unwrap();

    let mut stem = BufReader::new(conn);
    let mut buffer = Vec::new();
    stem.read_until(b'\n',&mut buffer).unwrap();

    let stc = String::from_utf8(buffer).unwrap();
    println!("stc: {}",stc);

    let tc: User = serde_json::from_str(stc.as_str()).unwrap();
    println!("Tc: {:?}",tc)

}
