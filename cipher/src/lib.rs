#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let expected_cipher = original
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                if c.is_ascii_lowercase() {
                    char::from_u32(219 - c as u32).unwrap()
                } else {
                    char::from_u32(155 - c as u32).unwrap()
                }
            } else {
                c
            }
        })
        .collect::<String>();

    if expected_cipher == ciphered {
        Ok(())
    } else {
        Err(CipherError {
            expected: expected_cipher,
        })
    }
}
