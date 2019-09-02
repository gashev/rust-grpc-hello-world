use futures::Future;
use grpcio::{
    Environment,
    ServerBuilder,
    UnarySink,
};
use std::{thread, time};
use std::sync::Arc;

#[path = "./../../../helloworld.rs"]
mod helloworld;

#[path = "./../../../helloworld_grpc.rs"]
mod helloworld_grpc;

use helloworld_grpc::{create_greeter, Greeter};
use helloworld::{
    HelloRequest,
    HelloReply
};

#[derive(Clone)]
struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(
        &mut self,
        ctx: ::grpcio::RpcContext,
        req: HelloRequest,
        sink: UnarySink<HelloReply>
    ) {
        println!("say_hello request");
        let msg = format!("Hello {}", req.get_name());
        let mut resp = HelloReply::default();
        resp.set_message(msg);
        let f = sink
            .success(resp)
            .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
        ctx.spawn(f)
    }
}

fn sleep() {
    loop {
        thread::sleep(time::Duration::from_millis(1000));
    };
}

fn main() {
    let service = create_greeter(GreeterService);
    let env = Arc::new(Environment::new(1));
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("0.0.0.0", 50051)
        .build()
        .unwrap();

    server.start();

    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }

    thread::spawn(sleep).join().unwrap();
}