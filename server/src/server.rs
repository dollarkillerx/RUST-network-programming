use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io::{Read, Write};
use std::time;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Server {
        Server{addr}
    }

    pub fn run(&self,sec: u8) -> std::io::Result<()> {
        println!("Bind: {} ",self.addr);
        let listen = TcpListener::bind(self.addr.clone())?;
        for conn in listen.incoming() {
            self.server(conn?,sec);
        }
        Ok(())
    }

    fn server(&self, mut conn: TcpStream,sec: u8)  {
        thread::spawn( move || {
            let mut buf = [0;512];

            loop {
                let byt_lan = match conn.read(&mut buf) {
                    Ok(len) => len,
                    Err(e) => {
                        println!("conn read err: {}",e);
                        continue;
                    },
                };
                if byt_lan == 0 {
                    return;
                }

                thread::sleep(time::Duration::from_secs(sec as u64));
                match conn.write(&buf[..byt_lan]) {
                    Err(e) => {
                        println!("conn write err [1]: {}",e);
                    },
                    Ok(_) => {},
                }

                match conn.write(&("\n".as_bytes())) {
                    Err(e) => {
                        println!("conn write err [2]: {}",e);
                    },
                    Ok(_) => {},
                }
            }
        });
    }
}