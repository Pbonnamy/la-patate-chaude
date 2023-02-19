use md5::{Md5, Digest};
use common::structs::{MD5HashCashInput, MD5HashCashOutput, ChallengeTrait};

pub struct MD5HashCash {
    input: MD5HashCashInput,
    #[allow(dead_code)]
    output: MD5HashCashOutput
}

fn hash(message: String) -> String {
    let mut hasher = Md5::new();
    hasher.update(message);
    let result = hasher.finalize();

    let hashcode = format!("{:032X}", result);

    hashcode
}


impl ChallengeTrait for MD5HashCash {

    type Input = MD5HashCashInput;
    
    type Output = MD5HashCashOutput;


    fn name() -> String {
        String::from("HashCash")
    }

    fn new(input: Self::Input) -> Self {
        MD5HashCash{ input, output: MD5HashCashOutput{ seed: 0, hashcode: "".to_string() } }
    }

    fn solve(&self) -> Self::Output {
        let input = self.input.message.clone();
        let mut seed = 0;
        let mut output: MD5HashCashOutput;
        
        loop {
            let hex_seed = format!("{:016X}", seed);

            let hashcode = hash(hex_seed + &input);

            output = MD5HashCashOutput{ seed, hashcode: hashcode.to_string() };

            if self.verify(&output) {
                break;
            }

            seed += 1;
        }

        output
    }

    fn verify(&self, output: &Self::Output) -> bool {
        let hex_integer = u128::from_str_radix(&output.hashcode, 16).unwrap();
        let leading_zero = hex_integer.leading_zeros();

        leading_zero >= self.input.complexity
    }
}