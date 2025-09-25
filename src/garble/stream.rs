use crate::garble::constants::CHARACTERS;

use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Debug)]
pub enum CipherError {
    EmptyField,
}

impl fmt::Display for CipherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CipherError::EmptyField => write!(f, "All fields must be filled out"),
        }
    }
}
pub fn stream_cipher(password: &str, pin: &str, salt: &str) -> Result<String, CipherError> {
    if password.is_empty() || pin.is_empty() {
        return Err(CipherError::EmptyField);
    }
    // Start PRNG From Seed
    let seed = format!("{}{}{}", pin, salt, password);
    let hash_seed = create_hash_seed(seed);
    let mut rng = SmallRng::from_seed(hash_seed);

    let final_password = encrypt(password, &mut rng);
    Ok(final_password)
}

fn encrypt(password: &str, rng: &mut SmallRng) -> String {
    let mut final_password = String::new();

    for char in password.chars() {
        let keystream_byte = rng.next_u32() as u8;
        let char_as_byte = char as u8;

        let encrypted_value = char_as_byte.wrapping_add(keystream_byte) % (CHARACTERS.len() as u8);
        let expansion_count = (encrypted_value % 3) + 1;

        for _ in 0..expansion_count {
            let output_keystream = rng.next_u32() as u8;
            let output_char_index = output_keystream as usize % CHARACTERS.len();
            final_password.push(CHARACTERS[output_char_index])
        }
    }

    return final_password;
}

fn create_hash_seed(seed: String) -> [u8; 32] {
    // Create a seed from the pin, salt, and password
    let mut hasher = Sha256::new();
    hasher.update(seed);
    let hash_result = hasher.finalize();

    // Start PRNG from hash
    let mut seed32 = [0u8; 32];
    seed32.copy_from_slice(&hash_result[..32]);

    return seed32;
}


