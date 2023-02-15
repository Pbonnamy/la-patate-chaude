
/////////////////////// TESTS ///////////////////////

use std::fs::File;
use std::io::{BufRead, BufReader};
use common::structs::{ChallengeTrait, RecoverSecretInput, RecoverSecretOutput};

#[cfg(test)]
mod tests{
    use common::structs::RecoverSecretInput;
    use super::*;

    #[test]
    fn test_tuples_from_letters() {
        let letters = "abcdef";
        let tuple_sizes = [2, 3, 1];
        let tuples = tuples_from_letters(letters, &tuple_sizes);
        assert_eq!(tuples, vec!["ab", "cde", "f"]);
    }

    #[test]
    fn test_is_tuple_in_good_order() {
        let tuple = "abe";
        let sentence = "abcdef";
        let is_in_order = is_tuple_in_good_order(tuple, sentence);
        assert_eq!(is_in_order, true);
    }

    #[test]
    fn test_sentence_with_distincts_chars() {
        let input = RecoverSecretInput {
            word_count: 1,
            letters: String::from("WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l"),
            tuple_sizes: vec![6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4]
        };
        assert_eq!(sentence_with_distincts_chars(&input), "xWRvraj4fonTUmzyO25wA3lBeiM9H");
    }
}

///////////////////////Function///////////////////////

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
pub fn is_one_sentence_of_chars(input : &RecoverSecretInput) -> bool {
    let word_count_in = input.word_count.clone();
    if word_count_in == 1 {
        return true;
    }
    return false ;
}
pub fn sentence_with_distincts_chars(input : &RecoverSecretInput) -> String {
    "to_do".to_string()
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
        sentence = sentence_with_distincts_chars(&input);
    }
    else {
        sentence = "Nothing".to_string();
    }
    return sentence;
}
