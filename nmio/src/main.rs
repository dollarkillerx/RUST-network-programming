use std::error::Error;

use mio::net::{TcpListener,TcpStream};
use mio::{Events,Interest,Poll,Token};

// 事件识别
const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn main() -> Result<(),Box<dyn Error>>{
    // Create a poll instance
    let mut poll = Poll::new()?;
    // Create storage for events  存放事件
    let mut events = Events::with_capacity(128);

    // Setup the server socket.
    let addr = "127.0.0.1:12345".parse()?;
    let mut server = TcpListener::bind(addr)?;

    //  注册监听  server,Token,事件
    poll.registry()
        .register(&mut server,SERVER,Interest::READABLE)?;

    let mut client = TcpStream::connect(addr)?;

    poll.registry()
        .register(&mut client,CLIENT,Interest::READABLE | Interest::WRITABLE)?;

    loop {
        // 获取事件 如果没有会被阻塞
        poll.poll(&mut events,None)?;

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    let conn = server.accept();
                    println!("收到数据");
                    drop(conn); // close conn
                },
                CLIENT => {
                    if event.is_writable() {
                        println!("is write");
                    }

                    if event.is_readable() {
                        println!("is read");
                    }

                    if event.is_read_closed() {
                        println!("read close");
                        return Ok(());
                    }
                    continue;
                },
                _ => unreachable!(),
            }
        }
    }
}
