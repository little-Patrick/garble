use crate::garble::constants::CHARACTERS;
use std::fmt;


#[derive(Debug)]
enum CipherError {
    InvalidPinLength,
    NonDigitInPin,
}

impl fmt::Display for CipherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CipherError::InvalidPinLength => write!(f, "PIN must be exactly 5 digits long"),
            CipherError::NonDigitInPin => write!(f, "PIN must contain only digits (0-9)"),
        }
    }

}


pub fn stream_cipher(password: &str, pin: &str) -> Result<String, CipherError> {
// 1) Pin
    // Get Pin
    // Add Salt to Pin
    // Run Stalt + Pin through hashing or keystring function

// 2) PRNG
    //  Use Hasing or keystring function from step 1 to start random seed stream

// 3) Password to Ord
    // Turn each charcter in password into ordinate value

// 4) XOR
    //  Take Password ordinate values from step 3 and XOR with Keystream from step 2
   
// 5) Map XOR Output
    // randomize CHARACTERS based on PIN + Salt
    // Map characters to XOR in step 4 by mapping each XOR Output % 94 = CHARACTERS index
    // TODO: While mapping stretch password encryption by deciding how many XOR values to take
    let 

}

