//! Module that contains the implementation of the RecoverSecret challenge
use common::functs::*;
use common::structs::*;

pub struct RecoverSecret {
    input: RecoverSecretInput,
    #[allow(dead_code)]
    output: RecoverSecretOutput,
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
                secret_sentence: String::new(),
            },
        }
    }

    fn solve(&self) -> Self::Output {
        let output = RecoverSecretOutput {
            secret_sentence: find_secret_sentence(&self.input),
        };

        output
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        verify_challenge(&self.input, answer)
    }
}

pub fn verify_challenge(input: &RecoverSecretInput, output: &RecoverSecretOutput) -> bool {
    let tuple_in = tuples_from_letters(&input.letters, &input.tuple_sizes);

    if input.word_count != nbr_of_words(&output.secret_sentence) {
        return false;
    }

    if are_tuples_in_good_order(&tuple_in, &output.secret_sentence) {
        return true;
    }

    return false;
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

    return false;
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

    return false;
}

pub fn is_one_sentence_of_chars(input: &RecoverSecretInput) -> bool {
    let word_count_in = input.word_count.clone();
    if word_count_in == 1 {
        return true;
    }
    return false;
}

pub fn word_with_distincts_chars(input: &RecoverSecretInput) -> String {
    let letters_in = input.letters.clone();
    let tuple_sizes_in = input.tuple_sizes.clone();

    let tuples = tuples_from_letters(&letters_in, &tuple_sizes_in);
    let sentence = build_word_of_unique_char(tuples.clone());

    sentence
}

//Cas des complexités supérieurs à 17 non traités
pub fn find_secret_sentence(input: &RecoverSecretInput) -> String {
    let sentence;

    if is_cest_chou(&input) {
        sentence = "C'est chou".to_string();
    } else if is_il_fait_froid(&input) {
        sentence = "Il fait froid".to_string();
    } else if is_one_sentence_of_chars(&input) {
        sentence = word_with_distincts_chars(&input);
    } else {
        //couldn't find a secret_sentence
        sentence = "Couldn't find a secret_sentence. Complexity too high !".to_string();
        //panic!("Couldn't find a secret_sentence. Complexity too high !");
    }
    return sentence;
}
//-----------------------------------------

//////////////////////////////////TESTS////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use common::structs::RecoverSecretInput;

    #[test]
    fn test_is_cest_chou() {
        let input = RecoverSecretInput {
            word_count: 2,
            letters: String::from("Cthou"),
            tuple_sizes: vec![1, 3, 1],
        };
        assert_eq!(is_cest_chou(&input), true);
    }

    #[test]
    fn test_is_il_fait_froid() {
        let input = RecoverSecretInput {
            word_count: 3,
            letters: String::from("Ilfaifro"),
            tuple_sizes: vec![2, 3, 3],
        };
        assert_eq!(is_il_fait_froid(&input), true);
    }

    #[test]
    fn test_is_one_sentence_of_chars() {
        let input = RecoverSecretInput {
            word_count: 1,
            letters: String::from("Chtou"),
            tuple_sizes: vec![1, 3, 1],
        };
        assert_eq!(is_one_sentence_of_chars(&input), true);
    }

    #[test]
    fn test_word_with_distincts_chars() {
        let input = RecoverSecretInput {
            word_count: 1,
            letters: String::from("Chtou"),
            tuple_sizes: vec![1, 3, 1, 1],
        };
        assert_eq!(word_with_distincts_chars(&input), "Chtou");
    }

    #[test]
    fn test_find_secret_sentence() {
        let input = RecoverSecretInput {
            word_count: 2,
            letters: String::from("Cthou"),
            tuple_sizes: vec![1, 3, 1],
        };
        assert_eq!(find_secret_sentence(&input), "C'est chou");
    }
    #[test]
    fn test_solve_challenge() {
        let input = RecoverSecretInput {
            word_count: 2,
            letters: "t cCehuCethoCeschouC'schout h".to_string(),
            tuple_sizes: vec![3, 4, 5, 7, 7, 3],
        };
        let recover_secret = RecoverSecret::new(input);
        assert_eq!(recover_secret.solve().secret_sentence, "C'est chou");
    }

    #[test]
    fn test_verify_challenge_complexity_0() {
        let input = RecoverSecretInput {
            word_count: 2,
            letters: "t cCehuCethoCeschouC'schout h".to_string(),
            tuple_sizes: vec![3, 4, 5, 7, 7, 3],
        };
        let output = RecoverSecretOutput {
            secret_sentence: "C'est chou".to_string(),
        };
        assert_eq!(verify_challenge(&input, &output), true);
    }

    #[test]
    fn test_verify_challenge_complexity_1to16() {
        let input = RecoverSecretInput {
            word_count: 1,
            letters: "WvyOAlxafUzleiSOl9xayBeHTmy9xWTU5lMW4nUO5lMWRajn2BiHSRUzy5afnUz5wlexWrm5wlBWr4mAlBrUmzHxTUzwlHrfTwBeSRmzlMSRfoUOAe9S4oUiraOiramzM5w3l".to_string(),
            tuple_sizes: vec![6, 8, 4, 6, 4, 7, 8, 9, 6, 9, 8, 7, 5, 7, 6, 6, 9, 5, 4, 5, 4]
        };
        let output = RecoverSecretOutput {
            secret_sentence: "xWvSRra4foTjnUmzyOA5w3l2Bei9HM".to_string(),
        };
        assert_eq!(verify_challenge(&input, &output), true);
    }
}
