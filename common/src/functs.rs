use std::fs::File;
use std::io::{BufRead, BufReader};

//--------------------------FILE FUNCTIONS----------------------------

///Return a vector of words from a file. Speically liste-mots-alpha.txt from data folder
/// Exemple: "data/liste-mots-alpha.txt" return ["a", "abaisser", "abandon", ...]
/// Note: The file must be in the same folder as the executable
pub fn words_from_file_list(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);
    let mut contents = vec![];

    //Recupere 1er mot de chaques lignes
    for line in reader.lines() {
        let l = &line.unwrap();
        let mut words = l.split_whitespace();
        contents.push(words.next().unwrap().to_string());
    }

    return contents;
}

//-------------------------STRING FUNCTIONS--------------------------

///Return a vector of tuples from a string. The size of the tuples are in the tuple_sizes vector
/// # Example
/// ```
/// use common::functs::tuples_from_letters;
/// let letters = "abracadabra";
/// let tuple_sizes = [2, 2, 2, 2, 2, 1];
/// let tuples = tuples_from_letters(letters, &tuple_sizes);
/// assert_eq!(tuples, ["ab", "ra", "ca", "da", "br", "a"]);
/// ```
pub fn tuples_from_letters(letters: &str, tuple_sizes: &[usize]) -> Vec<String> {
    let mut tuples = Vec::new();
    let mut index_letter = 0;
    let mut sizes = tuple_sizes.to_vec();

    while index_letter < letters.len() {
        let mut tuple = String::new();
        for _iter in 0..sizes[0] {
            tuple.push(letters.chars().nth(index_letter).unwrap());
            index_letter += 1;
        }
        sizes.remove(0);
        tuples.push(tuple);
    }
    tuples
}

/// Check if tuple is in good order in sentence. Return true if yes
/// # Example
/// ```
/// use common::functs::is_tuple_in_good_order;
/// let tuple = "abdb";
/// let sentence = "abracadabra";
/// let is_in_order = is_tuple_in_good_order(tuple, sentence);
/// assert_eq!(is_in_order, true);
/// ```
pub fn is_tuple_in_good_order(tuple: &str, sentence: &str) -> bool {
    let mut index_tuple = 0;
    let mut index_sentence = 0;
    let mut is_in_order = true;

    while index_tuple < tuple.len() && index_sentence < sentence.len() {
        if tuple.chars().nth(index_tuple).unwrap() == sentence.chars().nth(index_sentence).unwrap()
        {
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

/// Check if all tuples are in good order in sentence. Return true if yes
/// # Example
/// ```
/// use common::functs::are_tuples_in_good_order;
/// let tuples = ["abdb".to_string(), "brab".to_string(), "aca".to_string()];
/// let sentence = "abracadabra";
/// let are_in_order = are_tuples_in_good_order(&tuples, sentence);
/// assert_eq!(are_in_order, true);
/// ```
pub fn are_tuples_in_good_order(tuples: &[String], sentence: &str) -> bool {
    let mut are_in_order = true;
    for tuple in tuples {
        if !is_tuple_in_good_order(tuple, sentence) {
            are_in_order = false;
        }
    }
    are_in_order
}

/// Return the number of words in a sentence
/// # Example
/// ```
/// use common::functs::nbr_of_words;
/// let sentence = "abraca dab ra".to_string();
/// let nbr_words = nbr_of_words(&sentence);
/// assert_eq!(nbr_words, 3);
/// ```

pub fn nbr_of_words(sentence: &String) -> usize {
    let nbr_words = sentence.split_whitespace().count();
    nbr_words
}

/// Return the number of words in a sentence
/// # Example
/// ```
/// use common::functs::nbr_of_words;
/// let sentence = "abraca dab ra".to_string();
/// let nbr_words = nbr_of_words(&sentence);
/// assert_eq!(nbr_words, 3);
/// ```
pub fn is_char_in_sentence_in_order_of_tuple(ch: char, tuple: &String, sentence: &String) -> bool {
    // Si char pas dans tuple, alors erreur
    if !tuple.contains(ch) {
        panic!("Char not in tuple");
    }

    // Si char pas dans sentence, alors erreur
    if !sentence.contains(ch) {
        panic!("Char not in sentence");
    }

    //si char en debut de tuple, alors ok
    if tuple.chars().nth(0).unwrap() == ch {
        return true;
    }

    //si char en fin de tuple alors on prend tout le tuple
    if tuple.chars().nth(tuple.len() - 1).unwrap() == ch {
        if is_tuple_in_good_order(tuple, sentence) {
            return true;
        }
    } else {
        //Coupe tuple jusqu'au char inclus
        let tuple_cut_on_char = &tuple[0..tuple.find(ch).unwrap() + 1];
        if is_tuple_in_good_order(tuple_cut_on_char, sentence) {
            return true;
        }
    }
    return false;
}

/// Build a word from a vector of tuples with unique characters
pub fn build_word_of_unique_char(tuples: Vec<String>) -> String {
    let mut result = String::new();
    let tup = tuples.clone();

    while !are_tuples_in_good_order(&tup, &result) {
        for tuple in &tuples {
            for c in tuple.chars() {
                //Si pas dedans, on ajoute
                if !result.contains(c) {
                    result.push(c);
                } else {
                    //Si dedans
                    //Si apres lettre precedente du tuple, on ne fait rien
                    if is_char_in_sentence_in_order_of_tuple(c, &tuple, &result) {
                        continue;
                    } else {
                        //Si avant lettre precedente du tuple, on tronque
                        //char actuel
                        let index_actual_char = result.find(c).unwrap();

                        //char du debut de tuple
                        let index_first_char_of_tuple =
                            result.find(tuple.chars().nth(0).unwrap()).unwrap();

                        //char dans tuple juste avant l'actuel
                        let index_previous_char_of_tuple = result
                            .find(tuple.chars().nth(tuple.find(c).unwrap() - 1).unwrap())
                            .unwrap();
                        let previous_char_of_tuple =
                            result.chars().nth(index_previous_char_of_tuple).unwrap();

                        //Si char actuel avant char du debut de tuple, on tronque
                        if index_actual_char < index_first_char_of_tuple {
                            let seq_to_replace =
                                &result[index_actual_char..index_first_char_of_tuple];
                            let tmp = result.replace(seq_to_replace, "");
                            let res_deb = &tmp[0..tmp.find(previous_char_of_tuple).unwrap() + 1];
                            let res_fin;
                            //si precedent char du tuple est à la fin de tmp
                            if index_previous_char_of_tuple == tmp.len() - 1 {
                                res_fin = "";
                            } else {
                                res_fin = &tmp[tmp.find(previous_char_of_tuple).unwrap() + 1..];
                            }
                            result = res_deb.to_string()
                                + &*seq_to_replace.to_string()
                                + &*res_fin.to_string();
                        } else if index_actual_char > index_first_char_of_tuple
                            && index_actual_char < index_previous_char_of_tuple
                        {
                            // entre debut et fin de tuple
                            let seq_to_replace =
                                &result[index_actual_char..index_previous_char_of_tuple];
                            let tmp = result.replace(seq_to_replace, "");
                            let res_deb = &tmp[0..tmp.find(previous_char_of_tuple).unwrap() + 1];

                            let res_fin;
                            //si precedent char du tuple est à la fin de tmp
                            if index_previous_char_of_tuple == tmp.len() - 1 {
                                res_fin = "";
                            } else {
                                res_fin = &tmp[tmp.find(previous_char_of_tuple).unwrap() + 1..];
                            }
                            result = res_deb.to_string()
                                + &*seq_to_replace.to_string()
                                + &*res_fin.to_string();
                        }
                    }
                }
            }
        }
    }

    result
}

///////////////////////////TESTS///////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_tuple_in_good_order() {
        let tuple = "abc".to_string();
        let sentence = "abcgf".to_string();
        assert_eq!(is_tuple_in_good_order(&tuple, &sentence), true);
    }

    #[test]
    fn test_is_tuple_in_bad_order() {
        let tuple = "abc".to_string();
        let sentence = "bacrfsf".to_string();
        assert_eq!(is_tuple_in_good_order(&tuple, &sentence), false);
    }

    #[test]
    fn test_tuples_from_letters() {
        let letters = "t cCehuCethoCeschouC'schout h".to_string();
        let tuples = tuples_from_letters(&letters, &[3, 4, 5, 7, 7, 3]);
        assert_eq!(
            tuples,
            vec!["t c", "Cehu", "Cetho", "Ceschou", "C'schou", "t h"]
        );
    }

    #[test]
    fn test_are_tuples_in_good_order() {
        let tuples = vec![
            "t c".to_string(),
            "Cehu".to_string(),
            "Cetho".to_string(),
            "Ceschou".to_string(),
            "C'schou".to_string(),
            "t h".to_string(),
        ];
        let sentence = "t cCehuCethoCeschouC'schout h".to_string();
        assert_eq!(are_tuples_in_good_order(&tuples, &sentence), true);
    }

    #[test]
    fn test_nbr_of_words() {
        let letters = "je fonctionne correctement".to_string();
        let nbr_of_words = nbr_of_words(&letters);
        assert_eq!(nbr_of_words, 3);
    }

    #[test]
    fn test_nbr_of_word_empty() {
        let letters = "".to_string();
        let nbr_of_words = nbr_of_words(&letters);
        assert_eq!(nbr_of_words, 0);
    }

    #[test]
    fn test_nbr_of_word_multiple_space() {
        let letters = "je   fonctionne correctement".to_string();
        let nbr_of_words = nbr_of_words(&letters);
        assert_eq!(nbr_of_words, 3);
    }

    #[test]
    fn test_nbr_of_word_space_at_end() {
        let letters = "je fonctionne correctement ".to_string();
        let nbr_of_words = nbr_of_words(&letters);
        assert_eq!(nbr_of_words, 3);
    }

    #[test]
    fn test_nbr_of_word_space_at_beginning() {
        let letters = " je fonctionne correctement".to_string();
        let nbr_of_words = nbr_of_words(&letters);
        assert_eq!(nbr_of_words, 3);
    }

    #[test]
    fn test_nbr_of_word_space_at_beginning_and_end() {
        let letters = " je fonctionne correctement ".to_string();
        let nbr_of_words = nbr_of_words(&letters);
        assert_eq!(nbr_of_words, 3);
    }

    #[test]
    fn test_nbr_of_word_space_at_beginning_and_end_and_multiple_space() {
        let letters = " je fonctionne correctement ".to_string();
        let nbr_of_words = nbr_of_words(&letters);
        assert_eq!(nbr_of_words, 3);
    }

    #[test]
    fn test_nbr_of_word_space_at_beginning_and_end_and_multiple_space_and_uppercase() {
        let letters = " Je fonctionne correctement ".to_string();
        let nbr_of_words = nbr_of_words(&letters);
        assert_eq!(nbr_of_words, 3);
    }

    #[test]
    fn test_nbr_of_word_space_at_beginning_and_end_and_multiple_space_and_uppercase_and_special_char(
    ) {
        let letters = " Je fonctionne correctement !".to_string();
        let nbr_of_words = nbr_of_words(&letters);
        assert_eq!(nbr_of_words, 4);
    }

    #[test]
    fn test_nbr_of_word_multiple_following_space() {
        let letters = " Je fonctionne correctement !      ".to_string();
        let nbr_of_words = nbr_of_words(&letters);
        assert_eq!(nbr_of_words, 4);
    }
}
