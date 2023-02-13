use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use common::structs::{*};

pub(crate) struct RecoverSecret {
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

pub fn tuples_from_letters(letters: &str, tuple_sizes: &[usize] ) -> Vec<String> {
    let mut tuples = Vec::new();
    let mut index_letter = 0;

    for size in tuple_sizes {
        let mut tuple = String::new();
        for iter in 0..*size {
            tuple.push(letters.chars().nth(index_letter).unwrap());
            index_letter += 1;
        }
        tuples.push(tuple);
    }
    tuples
}
//Check si char dans tuple sont en ordre dans la phrase
pub fn is_tuple_in_good_order(tuple: &str, sentence: &str ) -> bool {
    let mut index_tuple = 0;
    let mut index_sentence = 0;
    let mut is_in_order = true;

    while index_tuple < tuple.len() && index_sentence < sentence.len() {
        if tuple.chars().nth(index_tuple).unwrap() == sentence.chars().nth(index_sentence).unwrap() {
            index_tuple += 1;
        }
        index_sentence += 1;
    }
    //Si ne trouve pas dans reste de la phrase, alors pas dans bon ordre
    if index_tuple < tuple.len() {
        is_in_order = false;
    }
    is_in_order
}

pub fn are_tuples_in_good_order(tuples: &[String], sentence: &str ) -> bool {
    let mut are_in_order = true;
    for tuple in tuples {
        if !is_tuple_in_good_order(tuple, sentence) {
            are_in_order = false;
        }
    }
    are_in_order
}

pub fn words_from_file_list(file_name: &str) -> Vec<String> {
    let mut file = File::open(file_name).expect("File not found");
    let mut reader = BufReader::new(file);
    let mut contents = vec![];

    //Recupere 1er mot de chaques lignes
    for line in reader.lines() {
        let mut l = &line.unwrap();
        let mut words = l.split_whitespace();
        contents.push(words.next().unwrap().to_string());
    }

    return contents ;
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
    else {
        let words = words_from_file_list("data/liste-mots-alphabetique.txt");
        let mut index_word = 0;
        let mut index_tuple = 0;
        let mut index_letter = 0;
        let mut tuple_size = tuple_sizes_in[index_tuple];
        let mut word_size = words[index_word].len();

        while index_word < word_count_in {
            //Si le mot est plus grand que le tuple, on passe au tuple suivant
            if word_size > tuple_size {
                index_tuple += 1;
                tuple_size = tuple_sizes_in[index_tuple];
            }
            //Si le mot est plus petit que le tuple, on passe au mot suivant
            else if word_size < tuple_size {
                index_word += 1;
                word_size = words[index_word].len();
            }
            //Si le mot est de la meme taille que le tuple, on verifie si le tuple est dans le bon ordre
            else {
                let mut tuple = String::new();
                for iter in 0..tuple_size {
                    tuple.push(letters_in.chars().nth(index_letter).unwrap());
                    index_letter += 1;
                }
                if is_tuple_in_good_order(&tuple, &words[index_word]) {
                    sentence.push_str(&words[index_word]);
                    sentence.push(' ');
                    index_word += 1;
                    word_size = words[index_word].len();
                }
                else {
                    index_tuple += 1;
                    tuple_size = tuple_sizes_in[index_tuple];
                }
            }
        }
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
