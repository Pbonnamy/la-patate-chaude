use common::structs::{RecoverSecretInput, RecoverSecretOutput, ChallengeTrait};

pub struct RecoverSecret {
    _input: RecoverSecretInput,
    _output: RecoverSecretOutput
}

impl ChallengeTrait for RecoverSecret {
    type Input = RecoverSecretInput;
    
    type Output = RecoverSecretOutput;

    fn name() -> String {
        String::from("RecoverSecret")
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