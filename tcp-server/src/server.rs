use std::io;
use std::net::TcpStream;
use std::io::{Read, Write};

pub fn server(mut conn: TcpStream) -> io::Result<()> {
    let mut buffer= [0;512];
    loop {
        let len = conn.read(&mut buffer)?;
        if len == 0 {
            return Ok(());
        }
        conn.write(&buffer[..])?;
        conn.flush()?;
    }
    // Ok(())
}