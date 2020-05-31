use server::client::Client;
use std::thread;
use futures;

fn main() {
    // the();  // 传统方式  线程  结果并行计算

    asy(); // async异步方式(在异步代码里使用阻塞 会使异步->同步)
}

fn the() -> std::io::Result<()> {

    let mut vc = Vec::new();

    let c = thread::spawn(|| {
        let _ = Client::run("127.0.0.1:8081","Hello this 1 8081");
    });
    vc.push(c);

    let c = thread::spawn(|| {
        let _ = Client::run("127.0.0.1:8081","Hello this 2 8081");
    });
    vc.push(c);


    let c = thread::spawn(|| {
        let _ = Client::run("127.0.0.1:8081","Hello this 3 8081");
    });
    vc.push(c);

    for i in vc {
        i.join().unwrap();
    }

    Ok(())
}


fn asy() {
    let f = seds();
    futures::executor::block_on(f);
}

async fn seds() {
    let f1 = sed("Hello World this one");
    let f2 = sed("Hello World this two");

    futures::join!(f1,f2);
}

async fn sed(msg: &str) {
    Client::run("127.0.0.1:8081",msg).unwrap();
}