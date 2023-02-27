use crate::structs::Message;
use std::io::prelude::*;
use std::net::TcpStream;

/// It sends a message to the server and use serde_json to serialize the message
/// # Arguments
/// * `stream` - A TcpStream containing the connection to the server
/// * `msg` - A Message struct containing the message to send
/// # Example
/// ```ignore
/// let mut stream = TcpStream::connect(address).unwrap();
/// let result = Message::Challenge(Challenge::MD5HashCash(input));
/// request::send_message(&mut stream, result);
/// ```
pub fn send_message(stream: &mut TcpStream, msg: Message) {
    let json = serde_json::to_string(&msg).unwrap();
    println!(
        "\n\x1b[34mDEBUG (Sended) : {}\x1b[0m",
        serde_json::to_string_pretty(&msg).unwrap()
    );
    let len = json.len() as u32;

    stream.write(&len.to_be_bytes()).unwrap();
    stream.write(&json.as_bytes()).unwrap();
}

/// It receives a message from the server and use serde_json to deserialize the message
/// # Arguments
/// * `stream` - A TcpStream containing the connection to the server
/// # Example
/// ```ignore
/// let mut stream = TcpStream::connect(address).unwrap();
/// let result = request::receive_message(&mut stream);
/// ```
/// # Output
/// A Message struct containing the message received
pub fn receive_message(stream: &mut TcpStream) -> Message {
    let mut buf_len = [0u8; 4];
    stream.read_exact(buf_len.as_mut()).unwrap();
    let len = u32::from_be_bytes(buf_len);

    let mut buf = vec![0u8; len as usize];
    stream.read_exact(buf.as_mut()).unwrap();
    let res = String::from_utf8_lossy(&buf);

    let json = serde_json::from_str(&res).unwrap();

    println!(
        "\n\x1b[35mDEBUG (Received) : {}\x1b[0m",
        serde_json::to_string_pretty(&json).unwrap()
    );

    return json;
}
