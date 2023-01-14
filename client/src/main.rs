mod recover_secret;
mod md5_hashcash;

use std::net::TcpStream;
use common::request;
use common::structs::{ Message, Subscribe };

fn main() {
    // Connect to server (need to run server first)

    // let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    //
    // request::send(&mut stream, Message::Hello);
    //
    // let subscribe = Subscribe { name: "test".to_string() };
    // request::send(&mut stream, Message::Subscribe(subscribe));

    //TEST
    let sentence = "Il fait froid" ;
    let letters = "IfflafIl froid";
    let tuples = recover_secret::tuples_from_letters(letters, &[3,3,8]);
    let are_in_order : bool = recover_secret::are_tuples_in_good_order(&tuples, sentence);
    println!("Tuples -> {:?}", tuples);
    println!("Tuples in order -> {:?}", are_in_order);
    let sentence = "C'est chou" ;
    //let are_unique : bool = recover_secret::are_random_unique_chars(sentence);
    //println!("Are unique -> {:?}", are_unique);
    let file_path = "data/liste-mots-alphabetique.txt";
    let words = recover_secret::words_from_file_list(file_path);
    println!("Words -> {:?}", words);

}