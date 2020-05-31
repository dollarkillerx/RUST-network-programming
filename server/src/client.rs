use std::net::{TcpStream};
use std::io::{prelude::*,BufReader,Write};


pub struct Client {}

impl Client {
    pub fn run(addr: &str,msg: &str) -> std::io::Result<()> {
        let mut conn = TcpStream::connect(addr)?;
        let _ = conn.write(msg.as_bytes())?;

        let mut reader = BufReader::new(&conn);
        let mut buffer: Vec<u8> = Vec::new();

        reader.read_until(b'\n', &mut buffer)?;
        println!("recv from server: {} ", std::str::from_utf8(&buffer).unwrap());
        Ok(())
    }
}