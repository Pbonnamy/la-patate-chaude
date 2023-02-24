use std::collections::HashMap;
use rand::Rng ;
use std::fs::File;
use std::io::{BufRead, BufReader};


//--------------------------FILE FUNCTIONS----------------------------

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

//-------------------------STRING FUNCTIONS--------------------------

pub fn tuples_from_letters(letters: &str, tuple_sizes: &[usize] ) -> Vec<String> {
    let mut tuples = Vec::new();
    let mut index_letter = 0;
    let mut sizes = tuple_sizes.to_vec();
    println!("tuple_sizes: {:?}", tuple_sizes);
    println!("letters: {:?}", letters) ;

    // for size in tuple_sizes {
    //     let mut tuple = String::new();
    //     for iter in 0..*size.clone() {
    //         println!("letters.chars().nth(index_letter).unwrap(): {:?}", letters.chars().nth(index_letter).unwrap());
    //         tuple.push(letters.chars().nth(index_letter).unwrap());
    //         index_letter += 1;
    //     }
    //     tuples.push(tuple);
    // }
    // tuples

    while index_letter < letters.len() {
        let mut tuple = String::new();
        for iter in 0..sizes[0] {
            tuple.push(letters.chars().nth(index_letter).unwrap());
            index_letter += 1;
        }
        sizes.remove(0);
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

pub fn nbr_of_words(sentence: &String) -> usize {
    let mut nbr_words = sentence.split_whitespace().count();
    nbr_words
}

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
    if tuple.chars().nth(tuple.len()-1).unwrap() == ch {
        println!("tuple_cut_on_char : {}", tuple) ;
        if is_tuple_in_good_order(tuple, sentence) {
            return true;
        }
    }else {
        //Coupe tuple jusqu'au char inclus
        let tuple_cut_on_char = &tuple[0..tuple.find(ch).unwrap() + 1];
        println!("tuple_cut_on_char : {}", tuple_cut_on_char) ;
        if is_tuple_in_good_order(tuple_cut_on_char, sentence) {
            return true;
        }
    }


    return false ;
}

pub fn build_word_of_unique_char(tuples: Vec<String>) -> String {
    let mut result = String::new();
    let mut tup = tuples.clone();

    while !are_tuples_in_good_order(&tup, &result) {
        println!("partial result: {:?}", result);
        for tuple in &tuples {
            for c in tuple.chars() {
                //Si pas dedans, on ajoute
                if !result.contains(c) {
                    result.push(c);
                } else { //Si dedans
                    //Si apres lettre precedente du tuple, on ne fait rien
                    if is_char_in_sentence_in_order_of_tuple(c, &tuple, &result) {
                        continue;
                    } else {  //Si avant lettre precedente du tuple, on tronque
                        //char actuel
                        let index_actual_char = result.find(c).unwrap();
                        let actual_char = result.chars().nth(index_actual_char).unwrap();

                        //char du debut de tuple
                        let index_first_char_of_tuple = result.find(tuple.chars().nth(0).unwrap()).unwrap();
                        let first_char_of_tuple = result.chars().nth(index_first_char_of_tuple).unwrap();

                        //char dans tuple juste avant l'actuel
                        let index_previous_char_of_tuple = result.find(tuple.chars().nth(tuple.find(c).unwrap() - 1).unwrap()).unwrap();
                        let previous_char_of_tuple = result.chars().nth(index_previous_char_of_tuple).unwrap();

                        //Si char actuel avant char du debut de tuple, on tronque
                        if index_actual_char < index_first_char_of_tuple {
                            let seq_to_replace = &result[index_actual_char..index_first_char_of_tuple];
                            let mut tmp = result.replace(seq_to_replace, "");
                            let mut res_deb = &tmp[0..tmp.find(previous_char_of_tuple).unwrap() + 1];
                            let mut res_fin;
                            //si precedent char du tuple est à la fin de tmp
                            if index_previous_char_of_tuple == tmp.len() - 1 {
                                res_fin = "";
                            } else {
                                res_fin = &tmp[tmp.find(previous_char_of_tuple).unwrap() + 1..];
                            }
                            result = res_deb.to_string() + &*seq_to_replace.to_string() + &*res_fin.to_string();
                        } else if index_actual_char > index_first_char_of_tuple && index_actual_char < index_previous_char_of_tuple { // entre debut et fin de tuple
                            let seq_to_replace = &result[index_actual_char..index_previous_char_of_tuple];
                            let mut tmp = result.replace(seq_to_replace, "");
                            let mut res_deb = &tmp[0..tmp.find(previous_char_of_tuple).unwrap() + 1];

                            let mut res_fin;
                            //si precedent char du tuple est à la fin de tmp
                            if index_previous_char_of_tuple == tmp.len() - 1 {
                                res_fin = "";
                            } else {
                                res_fin = &tmp[tmp.find(previous_char_of_tuple).unwrap() + 1..];
                            }
                            result = res_deb.to_string() + &*seq_to_replace.to_string() + &*res_fin.to_string();
                        }
                    }
                }
            }
        }
        println!("result : {}", result);
    }

    result
}

// pub fn build_word_of_unique_char(tuples: Vec<String>) -> String {
//     let mut result = String::new();
//     let mut char_map: HashMap<char, usize> = HashMap::new();
//
//     for (index, tuple) in tuples.iter().enumerate() {
//     for c in tuple.chars() {
//     if !char_map.contains_key(&c) {
//     char_map.insert(c, index);
//     }
//     }
//     }
//
//     let mut char_vec: Vec<(&char, &usize)> = char_map.iter().collect();
//     char_vec.sort_by(|a, b| a.1.cmp(b.1));
//
//     for (c, _) in char_vec {
//     result.push(*c);
//     }
//
//     result
// }














