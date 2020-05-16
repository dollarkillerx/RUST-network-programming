use std::thread;

use use_grpc::hello_grpc::*;
use use_grpc::hello::*;
use grpc::{ServerHandlerContext, ServerResponseUnarySink, ServerRequestSingle};

struct HelloWorldServer;

impl Hello for HelloWorldServer {
    fn hello_world(&self, _o: ServerHandlerContext, req: ServerRequestSingle<Req>, resp: ServerResponseUnarySink<Resp>) -> grpc::Result<()> {
        // 实例化返回
        let mut rp = Resp::new();
        // 获取数据
        println!("Req: {}",req.message.get_greeting());
        // 设置返回结果
        rp.set_reply(req.message.get_greeting().clone().to_string());
        // 刷入数据
        resp.finish(rp)
    }
}

fn main() {
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(9001);
    server.add_service(HelloServer::new_service_def(HelloWorldServer));

    let _server = server.build().expect("could not start server");
    loop {
        thread::park();
    }
}