use common::structs::{RecoverSecretInput, RecoverSecretOutput, ChallengeTrait};

struct RecoverSecret {
    input: RecoverSecretInput,
    output: RecoverSecretOutput
}

impl ChallengeTrait for RecoverSecret {
    type Input = RecoverSecretInput;
    
    type Output = RecoverSecretOutput;

    fn name() -> String {
        String::from("RecoverSecret")
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