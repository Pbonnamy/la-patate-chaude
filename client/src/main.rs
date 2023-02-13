mod recover_secret;
mod md5_hashcash;

use std::net::TcpStream;
use common::request;
use common::structs::{Message, Subscribe, RecoverSecretInput, ChallengeTrait};
use common::structs::ChallengeAnswer::RecoverSecret;
use crate::recover_secret::{*};


fn main() {
    // Connect to server (need to run server first)

    // let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    //
    // request::send(&mut stream, Message::Hello);
    //
    // let subscribe = Subscribe { name: "test".to_string() };
    // request::send(&mut stream, Message::Subscribe(subscribe));

    //TEST
    /*let recover_input = RecoverSecretInput {
        word_count : 2,
        letters : "t cCehuCethoCeschouC'schout h".to_string(),
        tuple_sizes : vec![3, 4, 5, 7, 7, 3]
    };*/
    // let recover_input = RecoverSecretInput {
    //     word_count : 2,
    //     letters : "Cchou".to_string(),
    //     tuple_sizes : vec![1,4]
    // };
    // let recover_input = RecoverSecretInput {
    //     word_count : 3,
    //     letters : "IfflfaIlfroid".to_string(),
    //     tuple_sizes : vec![3,3,7]
    // };
     let recover_input = RecoverSecretInput {
         word_count : 2,
         letters : "lvlle".to_string(),
         tuple_sizes : vec![3,2]
     };
    //let challenge_recover = recover_secret::RecoverSecret::new(recover_input);
    // let sentence = "Il fait froid" ;
    // let letters = "IfflafIl froid";
    // let tuples = recover_secret::tuples_from_letters(letters, &[3,3,8]);
    // let are_in_order : bool = recover_secret::are_tuples_in_good_order(&tuples, sentence);
    // println!("Tuples -> {:?}", tuples);
    // println!("Tuples in order -> {:?}", are_in_order);
    // let sentence = "C'est chou" ;
    //let are_unique : bool = recover_secret::are_random_unique_chars(sentence);
    //println!("Are unique -> {:?}", are_unique);
    // let file_path = "data/liste-mots-alphabetique.txt";
    // let words = recover_secret::words_from_file_list(file_path);
    // println!("Words -> {:?}", words);
    //let is_cest_chou_test = is_cest_chou(recover_input) ;
    //let is_il_fait_froid_test = recover_secret::is_il_fait_froid(recover_input) ;
    //println!("Is c'est chou -> {:?}", is_cest_chou_test);
    //println!("Is il fait froid -> {:?}", is_il_fait_froid_test);
    let secret_sentence = secret_sentence(&recover_input);
    println!("Secret sentence -> {:?}", secret_sentence);
}