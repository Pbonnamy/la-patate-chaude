mod md5_hashcash;
mod recover_secret;

use crate::recover_secret::*;
use common::functs::*;
use common::request;
use common::structs::ChallengeAnswer::RecoverSecret;
use common::structs::{ChallengeTrait, Message, RecoverSecretInput, Subscribe};
use std::net::TcpStream;

fn main() {
    // Connect to server (need to run server first)

    // let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    //
    // request::send(&mut stream, Message::Hello);
    //
    // let subscribe = Subscribe { name: "test".to_string() };
    // request::send(&mut stream, Message::Subscribe(subscribe));

    //TEST
    // let recover_input = RecoverSecretInput {
    //     word_count : 2,
    //     letters : "t cCehuCethoCeschouC'schout h".to_string(),
    //     tuple_sizes : vec![3, 4, 5, 7, 7, 3]
    // };
    // let recover_input = RecoverSecretInput {
    //     word_count : 2,
    //     letters : "Cchou".to_string(),
    //     tuple_sizes : vec![1,4]
    // };
    // let recover_input = RecoverSecretInput {
    //     word_count : 3,
    //     letters : "IfflfaIlfroid".to_string(),
    //     tuple_sizes : vec![3,3,7]
    // // };
    let recover_input = RecoverSecretInput {
        word_count: 1,
        letters: "WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l".to_string(),
        tuple_sizes: vec![6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4]
    };
    //let challenge_recover = recover_secret::RecoverSecret::new(recover_input);
    // let sentence = "Il fait froid" ;
    // let letters = "IfflafIl froid";
    //let is_char_in_sentence = is_char_in_sentence_in_order_of_tuple('z', &"jaz".to_string(), &"je mange".to_string());
    let tuples = tuples_from_letters(
        &recover_input.letters,
        &[
            6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4,
        ],
    );
    //let tuples = tuples_from_letters(&recover_input.letters, &[3, 4, 5, 7, 7, 3]);
    //println!("Is char in sentence -> {:?}", is_char_in_sentence) ;
    // let are_in_order : bool = recover_secret::are_tuples_in_good_order(&tuples, sentence);
    println!("Tuples -> {:?}", tuples);
    // println!("Tuples in order -> {:?}", are_in_order);
    // let sentence = "C'est chou" ;
    //let are_unique : bool = recover_secret::are_random_unique_chars(sentence);
    //println!("Are unique -> {:?}", are_unique);
    // let char_utf8 = convert_char('�');
    // println!("Char utf8 -> {:?}", char_utf8);
    //  let file_path = "data/liste-mots-alphabetique.txt";
    //  let words = words_from_file_list(file_path);
    //  println!("Words -> {:?}", words);
    //let is_cest_chou_test = is_cest_chou(recover_input) ;
    //let is_il_fait_froid_test = recover_secret::is_il_fait_froid(recover_input) ;
    //println!("Is c'est chou -> {:?}", is_cest_chou_test);
    //println!("Is il fait froid -> {:?}", is_il_fait_froid_test);
    //let secret_sentence = secret_sentence(&recover_input);
    // let char_in_order = is_char_in_sentence_in_order_of_tuple('z', &"jaz".to_string(), &"je mange".to_string());
    // println!("Char in order -> {:?}", char_in_order);
    //let word = build_word_of_unique_char(tuples) ;
    //let good_word = change_order_of_letters(&word);
    //println!("Word -> {:?}", word);
    //  println!("Good word -> {:?}", good_word);
    //println!("Secret sentence -> {:?}", secret_sentence);
    let mut word_unique = word_with_distincts_chars(&recover_input);
    println!("Word unique -> {:?}", word_unique);
    let sent = "xWvSRra4foTjnUmzyOA5w3l2Bei9HM".to_string();
    let isvalid = are_tuples_in_good_order(&tuples, &sent);
    println!("Is valid -> {:?}", isvalid);
}
