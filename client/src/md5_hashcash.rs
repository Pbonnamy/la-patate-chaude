use md5::{Md5, Digest};
use std::{str};
use std::fmt::{Error, Write};
use common::structs::{MD5HashCashInput, MD5HashCashOutput, ChallengeTrait};

pub struct MD5HashCash {
    _input: MD5HashCashInput,
    _output: MD5HashCashOutput
}

fn hash(message: &str) -> String{

    let mut hasher = Md5::new();
    hasher.update(message.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn count_bits_zero(string: &str) -> u64{
    //println!("String: {}", string);
    let mut count = 0;
    let decode = hex::decode(string).expect("Erreur au decoding de la chaine");
    let binary =
        String::from_utf8_lossy(&decode);
    for i in binary.chars() {
        if i == '0' {
            count += 4;
        }else if i == '1' {
            count += 3;
        }else if i == '2' || i == '3' {
            count += 2;
        }else if i == '4' || i == '5' || i == '6' || i == '7' {
            count += 1;
        }
    }
    count

}

fn verifyHashCode(message: &str, seed: u64) -> Result<String, Error>{
    let seed: u64 = seed;
    let mut hex_seed = String::new();
    write!(&mut hex_seed, "{:016x}", seed).expect("Erreur à la conversion du seed en hexa");
    let hash_code = hash(&(hex_seed + &message));
    return Ok(hash_code.to_uppercase());
}


impl ChallengeTrait for MD5HashCash {

    type Input = MD5HashCashInput;
    
    type Output = MD5HashCashOutput;


    fn name() -> String {
        String::from("HashCash")
    }

    fn new(_input: Self::Input) -> Self {
        MD5HashCash{ _input, _output: MD5HashCashOutput{ seed: 0, hashcode: "".to_string() } }
    }

    fn solve(&self) -> Self::Output {
        let mut seed: u64 = 0;
        let mut hex_seed = String::new();
        write!(&mut hex_seed, "{:016x}", seed).expect("Erreur à la conversion du seed en hexa");
        let mut hash_code = hash(&(hex_seed + &self._input.message));
        let mut bits_to_zero = count_bits_zero(&hash_code);
        let input: &MD5HashCashInput = &self._input;
        while bits_to_zero < input.complexity as u64 {
            seed += 1;
            let mut hex_seed = String::new();
            write!(&mut hex_seed, "{:016x}", seed).expect("Erreur à la conversion du seed en hexa");
            hash_code = hash(&(hex_seed + &self._input.message));
            bits_to_zero = count_bits_zero(&hash_code);
        }
        let output = MD5HashCashOutput{ seed, hashcode: hash_code.to_uppercase() };

        if self.verify(&output) {
            return output;
        } else {
            panic!("Erreur dans la vérification du hashcash")
        }
        /*&self._output.unwrap().seed = output.seed;
        &self._output.unwrap().hashcode = output.hashcode.to_string();
        self._output.unwrap()*/
    }

    fn verify(&self, _output: &Self::Output) -> bool {
        let seed: u64 = _output.seed;
        return match verifyHashCode(&self._input.message, seed) {
            Ok(value) => {
                if value == _output.hashcode {
                    println!("Ok: {}", value);
                    true
                } else {
                    println!("La valeur ne correspond pas: {}", value);
                    false
                }
            }
            Err(e) => {
                println!("Erreur dans la vérification du hashcash: {}", e);
                false
            }
        }
    }
}