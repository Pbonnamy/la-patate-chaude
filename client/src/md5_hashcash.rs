use md5::digest::Output;
use md5::Md5;
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

    let binary = str::from_utf8(&hex::decode(string).expect("Erreur au decoding de la chaine")).expect("Erreur de conversion");

    let mut count = 0;
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

impl ChallengeTrait for MD5HashCash {

    type Input = MD5HashCashInput;
    
    type Output = MD5HashCashOutput;


    fn name() -> String {
        String::from("HashCash")
    }

    fn new(_input: Self::Input) -> Self {
        let mut hex_seed = String::new();
        let seed: u64 = 0;
        write!(&mut hex_seed, "{:016x}", seed).expect("Erreur à la conversion du seed en hexa");
        let hash = hash(&(hex_seed + &*_input.message));
    }

    fn solve(&self) -> Self::Output {
        let seed: u64 = 0;
        write!(&mut hex_seed, "{:016x}", seed).expect("Erreur à la conversion du seed en hexa");
        let hash = hash(&(hex_seed + &*_input.message));
        let bits_to_zero = count_bits_zero(&hash);
        let _input: MD5HashCashInput = Self::Input;
        if bits_to_zero >= _input.complexity as u64 {
            &self._output.seed = &seed;
            &self._output.hashcode = &hash;
        }
        Self::Output = &self._output;
        return Output

    }

    fn verify(&self, _output: &Self::Output) -> bool {
        todo!()
    }
}