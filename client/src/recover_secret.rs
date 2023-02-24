use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::ops::Index;
use common::structs::{*};
use common::functs::*;

pub struct RecoverSecret {
    input: RecoverSecretInput,
    output: RecoverSecretOutput
}

impl ChallengeTrait for RecoverSecret {
    type Input = RecoverSecretInput;
    
    type Output = RecoverSecretOutput;

    fn name() -> String {
        String::from("RecoverSecret")
    }

    fn new(input: Self::Input) -> Self {
        RecoverSecret {
            input,
            output: RecoverSecretOutput {
                secret_sentence: String::new()
            }
        }
    }

    fn solve(&self) -> Self::Output {
        todo!()
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}

pub fn verify_challenge(input: &RecoverSecretInput, output: &RecoverSecretOutput) -> bool {
    let mut tuple_in = tuples_from_letters(&input.letters, &input.tuple_sizes);

    if input.word_count != nbr_of_words(&output.secret_sentence) {
        return false;
    }
    // Gerer cas ou il y a 1 seul mot avec char distincts. are_tuples in good order ne valide pas tout

    if are_tuples_in_good_order(&tuple_in, &output.secret_sentence) {
        return true;
    }


    return false ;

}

pub fn is_cest_chou(input: &RecoverSecretInput) -> bool {
    let sentence_default = "C'est chou";
    let word_count_in = input.word_count.clone();
    let letters_in = input.letters.clone();
    let tuple_sizes_in = input.tuple_sizes.clone();

    let tuples = tuples_from_letters(&letters_in, &tuple_sizes_in);
    let in_order = are_tuples_in_good_order(&tuples, &sentence_default);

    if in_order && word_count_in == 2 {
        return true;
    }

    return false ;
}

pub fn is_il_fait_froid(input: &RecoverSecretInput) -> bool {
    let sentence_default = "Il fait froid";
    let word_count_in = input.word_count.clone();
    let letters_in = input.letters.clone();
    let tuple_sizes_in = input.tuple_sizes.clone();

    let tuples = tuples_from_letters(&letters_in, &tuple_sizes_in);
    let in_order = are_tuples_in_good_order(&tuples, &sentence_default);

    if in_order && word_count_in == 3 {
        return true;
    }

    return false ;
}

pub fn is_one_sentence_of_chars(input : &RecoverSecretInput) -> bool {
    let word_count_in = input.word_count.clone() ;
    if word_count_in == 1 {
        return true;
    }
    return false ;
}

pub fn word_with_distincts_chars(input : &RecoverSecretInput) -> String {
    let mut sentence = String::new();
    let word_count_in = input.word_count.clone();
    let letters_in = input.letters.clone();
    let tuple_sizes_in = input.tuple_sizes.clone();

    let mut tuples = tuples_from_letters(&letters_in, &tuple_sizes_in);
    sentence = build_word_of_unique_char(tuples.clone());

    sentence
}




pub fn secret_sentence( input: &RecoverSecretInput) -> String {
    let mut sentence = String::new();
    let word_count_in = input.word_count.clone();
    let letters_in = input.letters.clone();
    let tuple_sizes_in = input.tuple_sizes.clone();

    let tuples = tuples_from_letters(&letters_in, &tuple_sizes_in);

    if is_cest_chou(&input) {
        sentence = "C'est chou".to_string();
    }
    else if is_il_fait_froid(&input) {
        sentence = "Il fait froid".to_string();
    }
    else if is_one_sentence_of_chars(&input){
        sentence = word_with_distincts_chars(&input);
    }
    else {
        //couldn't find a secret_sentence
        sentence = "Couldn't find a secret_sentence. Complexity too high !".to_string();
        //panic!("Couldn't find a secret_sentence. Complexity too high !");
    }
    return sentence;
}
//-----------------------------------------








//pub fn words_from_data(data: )

/*pub fn generate_sentence_from_tuples(tuple: &str) -> String {
    // Use word in liste-mots-alphabetique.txt


}*/

/* //Checker complexité pour voir comment on genere le complexity
pub fn complexity_of(sentence : &str) -> u32{
    let value : u32 ;

    match(sentence) {
        "C'est chou" => value = 0,
        "Il fait froid" => value = 17,
        //Superieur a 17
        _ => value = 18,
    }
    //Si la phrase est uniquement faite de char, sa complexité vaut entre 1 et 16
    if(are_random_unique_chars(sentence)){
        //nombre aleatoire entre 1 et 16
        return 5
    }

    return value;
}
pub fn are_random_unique_chars(sentence : &str) -> bool {
    let mut are_unique = true;
    let mut index = 0;
    while index < sentence.len() && are_unique {
        let mut index2 = index + 1;
        while index2 < sentence.len() && are_unique {
            if sentence.chars().nth(index).unwrap() == sentence.chars().nth(index2).unwrap() {
                are_unique = false;
            }
            index2 += 1;
        }
        index += 1;
    }
    are_unique
}*/














///////////////////////////////////////////TESTS///////////////////////////////////////////

// pub fn recover_secret_test_complexity_0() {
//     let input = RecoverSecretInput {
//         word_count: 2,
//         letters: String::from("t cCehuCethoCeschouC'schout h"),
//         tuple_sizes: vec![3, 4, 5, 7, 7, 3]
//     };
//
//     let challenge = RecoverSecret::new(input);
//     let output = challenge.solve();
//     let is_valid = challenge.verify(&output);
// }
//
// pub fn recover_secret_test_complexity_1to16(){
//     let input = RecoverSecretInput {
//         word_count: 1,
//         letters: String::from("WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l"),
//         tuple_sizes: vec![6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4]
//     };
//
//     let challenge = RecoverSecret::new(input);
//     let output = challenge.solve();
//     let is_valid = challenge.verify(&output);
// }
//
// pub fn recover_secret_test_complexity_17() {
//     let input = RecoverSecretInput {
//         word_count: 3,
//         letters: String::from("ifflafilfroid"),
//         tuple_sizes: vec![3,3,7]
//     };
//
//     let challenge = RecoverSecret::new(input);
//     let output = challenge.solve();
//     let is_valid = challenge.verify(&output);
// }
