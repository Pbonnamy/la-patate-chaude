use common::structs::{MD5HashCashInput, MD5HashCashOutput, ChallengeTrait};

struct HashCash{
    _input: MD5HashCashInput,
    _output: MD5HashCashOutput
}

impl ChallengeTrait for HashCash {

    type Input = MD5HashCashInput;
    
    type Output = MD5HashCashOutput;

    fn name() -> String {
        String::from("HashCash")
    }

    fn new(_input: Self::Input) -> Self {
        todo!()
    }

    fn solve(&self) -> Self::Output {
        todo!()
    }

    fn verify(&self, _output: &Self::Output) -> bool {
        todo!()
    }
}