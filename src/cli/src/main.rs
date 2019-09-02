use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

#[path = "./../../../helloworld.rs"]
mod helloworld;

#[path = "./../../../helloworld_grpc.rs"]
mod helloworld_grpc;

use helloworld_grpc::GreeterClient;

use helloworld::{
    HelloRequest,
    HelloReply
};

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("server:50051");
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::default();
    req.set_name("world".to_owned());
    let reply: HelloReply = client.say_hello(&req).expect("rpc");
    println!("Greeter received: {}", reply.get_message());
}