mod md5_hashcash;
mod recover_secret;

use std::net::TcpStream;
use common::request;
use common::structs::{ Message, Subscribe };

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    request::send(&mut stream, Message::Hello);

    let subscribe = Subscribe { name: "test".to_string() };
    request::send(&mut stream, Message::Subscribe(subscribe));
}