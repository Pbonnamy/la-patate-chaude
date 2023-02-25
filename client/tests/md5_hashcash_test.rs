#[cfg(test)]
mod tests {
    use md5::{Digest, Md5};

    #[test]
    fn test_md5_string() {
        let message = "0000000000000131Our boring client talks to a red computer.";
        let mut hasher = Md5::new();
        hasher.update(message.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        assert_eq!(hash, "07A823F1314F193158E1A3302ECE66D6".to_lowercase());
        assert_eq!(hash.to_uppercase(), "07A823F1314F193158E1A3302ECE66D6");
    }

    #[test]
    fn test_md5_string_wrong() {
        let message = "0000000000000131Our boring client talks to a red computer.";
        let mut hasher = Md5::new();
        hasher.update(message.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        assert_ne!(hash, "07A823F1314F193158E1A3302ECE66D7");
        assert_ne!(hash, "07A823F1314F193158E1A3302ECE66D7".to_lowercase());
        assert_ne!(hash.to_uppercase(), "07A823F1314F193158E1A3302ECE66D7");
    }
}
