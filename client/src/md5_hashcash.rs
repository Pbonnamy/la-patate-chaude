use common::structs::{MD5HashCashInput, MD5HashCashOutput, ChallengeTrait};
use md5::{Md5, Digest};

struct HashCash{
    input: MD5HashCashInput,
    output: MD5HashCashOutput
}

impl Challenge for HashCash {

    type Input = MD5HashCashInput;
    
    type Output = MD5HashCashOutput;

    fn name() -> String {
        String::from("HashCash")
    }

    fn new(input: Input) -> Self {
        todo!()
    }

    fn solve(&self) -> Self::Output {
        todo!()
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}