use common::structs::{ChallengeTrait, MD5HashCashInput, MD5HashCashOutput};
use md5::{Digest, Md5};

struct HashCash {
    input: MD5HashCashInput,
    output: MD5HashCashOutput,
}

impl ChallengeTrait for HashCash {
    type Input = MD5HashCashInput;

    type Output = MD5HashCashOutput;

    fn name() -> String {
        String::from("HashCash")
    }

    fn new(input: Self::Input) -> Self {
        todo!()
    }

    fn solve(&self) -> Self::Output {
        todo!()
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        todo!()
    }
}
