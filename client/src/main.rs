mod md5_hashcash;
mod recover_secret;

use std::net::TcpStream;
use common::request;
use common::structs::{ Message, Subscribe };

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    request::send_message(&mut stream, Message::Hello);

    loop {
        let response = request::receive_message(&mut stream);

        match response {
            Message::Welcome(_welcome) => {
                let subscribe = Subscribe { name: "test".to_string() };
                request::send_message(&mut stream, Message::Subscribe(subscribe));
            },
            _ => {}
        }
    }
}