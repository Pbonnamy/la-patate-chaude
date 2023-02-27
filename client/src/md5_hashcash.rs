use common::structs::{ChallengeTrait, MD5HashCashInput, MD5HashCashOutput};
use md5::{Digest, Md5};

/// A challenge that requires the solver to find a seed that produces a hashcode with a given number of leading zeros.
/// Le challenge consiste à trouver un seed qui produit un hashcode avec un nombre donné de zéros en tête.
/// Las structure est définit de la manière suivante
pub struct MD5HashCash {
    /// L'input du challenge est de type MD5HashCashInput
    input: MD5HashCashInput,
    #[allow(dead_code)]
    /// L'output du challenge est de type MD5HashCashOutput
    output: MD5HashCashOutput,
}

/// Fonction qui calcule le hashcode d'un message
/// La fonction prend en paramètre un message de type String
/// # Arguments
/// * `message` - Le message à hasher
/// # Example
/// ```
/// let hashcode = hash("0000000000000131Our boring client talks to a red computer.");
/// ```
/// # Output
/// Un hashcode de type String qui est le résultat du hashage du message en hexadécimal
fn hash(message: String) -> String {
    let mut hasher = Md5::new();
    hasher.update(message);
    let result = hasher.finalize();

    let hashcode = format!("{:032X}", result);

    hashcode
}

/// Challenge Md5HashCash qui implémente le trait ChallengeTrait
impl ChallengeTrait for MD5HashCash {
    /// Le type d'input est MD5HashCashInput
    type Input = MD5HashCashInput;

    /// Le type d'output est MD5HashCashOutput
    /// # Exemple
    /// ```
    /// fn solve(&self) -> Self::Output{ return self.output;}
    /// ```
    type Output = MD5HashCashOutput;

    fn name() -> String {
        String::from("HashCash")
    }

    /// Fonction qui retourne une nouvelle instance de MD5HashCash
    /// # Arguments
    /// * `input` - Le message à hasher + la complexité du challenge de type MD5HashCashInput
    /// # Exemple
    /// ```
    /// use md5_hashcash::MD5HashCash;
    /// let challenge = MD5HashCash::new(input);
    /// ```
    fn new(input: Self::Input) -> Self {
        MD5HashCash {
            input,
            output: MD5HashCashOutput {
                seed: 0,
                hashcode: "".to_string(),
            },
        }
    }

    /// Fonction qui résout le challenge
    /// # Exemple
    /// ```
    /// use md5_hashcash::MD5HashCash;
    /// let challenge = MD5HashCash::new(input);
    /// let output = challenge.solve();
    /// ```
    /// # Output
    /// Le seed qui a permis de trouver le hashcode avec la complexité demandée de type MD5HashCashOutput
    fn solve(&self) -> Self::Output {
        let input = self.input.message.clone();
        let mut seed = 0;
        let mut output: MD5HashCashOutput;

        loop {
            let hex_seed = format!("{:016X}", seed);

            let hashcode = hash(hex_seed + &input);

            output = MD5HashCashOutput {
                seed,
                hashcode: hashcode.to_string(),
            };

            if self.verify(&output) {
                break;
            }

            seed += 1;
        }

        output
    }

    /// Fonction qui vérifie si le hashcode trouvé est valide
    /// # Arguments
    /// * `output` - Le hashcode trouvé de type MD5HashCashOutput
    /// # Exemple
    /// ```
    /// use md5_hashcash::MD5HashCash;
    /// let challenge = MD5HashCash::new(input);
    /// let output = challenge.solve();
    /// let is_valid = challenge.verify(&output);
    /// ```
    /// # Output
    /// True si le hashcode trouvé est valide, False sinon
    fn verify(&self, output: &Self::Output) -> bool {
        let hex_integer = u128::from_str_radix(&output.hashcode, 16).unwrap();
        let leading_zero = hex_integer.leading_zeros();

        leading_zero >= self.input.complexity
    }
}

#[cfg(test)]
mod tests {
    use crate::md5_hashcash::MD5HashCash;
    use common::structs::{ChallengeTrait, MD5HashCashInput};

    #[test]
    fn test_hashcash() {
        let input = MD5HashCashInput {
            complexity: 0,
            message: "Hello World".to_string(),
        };
        let hash_cash = MD5HashCash::new(input);
        let output = hash_cash.solve();
        assert!(hash_cash.verify(&output));
    }

    #[test]
    fn test_hex_seed(){
        let seed = 789;
        let hex_seed = format!("{:016X}", seed);
        assert_eq!(hex_seed, "0000000000000315");
    }

    #[test]
    fn test_hash(){
        let message = "Hello World".to_string();
        let hashcode = super::hash(message);
        assert_eq!(hashcode, "B10A8DB164E0754105B7A99BE72E3FE5");
    }

    #[test]
    fn test_count_bits_zero(){
        let hex_integer = u128::from_str_radix("ED076287532E86365E841E92BFC50D8C", 16).unwrap();
        let leading_zero = hex_integer.leading_zeros();
        assert_eq!(leading_zero, 0);
    }

}
