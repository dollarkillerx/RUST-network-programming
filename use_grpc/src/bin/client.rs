use use_grpc::hello::*;
use use_grpc::hello_grpc::*;
use futures::executor;

use grpc::ClientStubExt;

fn main() {
    let client = HelloClient::new_plain(
        "127.0.0.1",
        9001,
        Default::default(),
    ).unwrap();

    let mut req = Req::new();
    req.set_greeting("bac".to_string());

    let resp_fun = client
        .hello_world(
        grpc::RequestOptions::new(),
        req,
    ).join_metadata_result();
    let resp = executor::block_on(resp_fun);
    match resp {
        Err(e) => panic!("{:?}",e),
        Ok((_,cabs,_)) => {
            let c = cabs as Resp;
            println!("resp: {:?}", c.reply)
        },
    }
}