mod base64;

use base64::Base64;
fn main() {
    let original = "ab";
    let mut base64 = Base64::new();

    let encoded = match base64.encode(original) {
        Ok(e) => e,
        Err(e) => panic!("{}", e),
    };

    let decoded = match base64.decode(&encoded) {
        Ok(d) => d,
        Err(e) => panic!("{}", e),
    };

    println!("Original: {}", original);
    println!("Encoded: {}", encoded);
    println!("Decoded: {}", decoded);

    assert_eq!(original, decoded, "Failed");
    assert!(
        original.eq(decoded.as_str()),
        "Original and decoded are not equal!"
    );
}

#[cfg(test)]
mod tests {
    use super::base64::Base64;

    #[test]
    fn test_base64_encode_decode() {
        let original = "hello world";
        let mut base64 = Base64::new();

        let encoded = base64.encode(original);
        let decoded = base64.decode(&encoded.unwrap());

        assert_eq!(
            original,
            decoded.unwrap(),
            "Original and decoded messages do not match"
        );
    }
}
