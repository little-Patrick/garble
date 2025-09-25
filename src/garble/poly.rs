use crate::garble::constants::CHARACTERS;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Lane {
    A,
    B,
    C,
    D,
}

impl Lane {
    /// Which lane (A..D) based on an index in the password.
    fn from_index(i: usize) -> Self {
        match i % 4 {
            0 => Lane::A,
            1 => Lane::B,
            2 => Lane::C,
            _ => Lane::D,
        }
    }
}

#[derive(Debug)]
pub enum CipherError {
    InvalidPinLength,
    NonDigitInPin,
    ParseError,
    MissingLaneShift(Lane),
    Internal(&'static str),
}

impl fmt::Display for CipherError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CipherError::InvalidPinLength => write!(f, "PIN must be exactly 5 digits long"),
            CipherError::NonDigitInPin => write!(f, "PIN must contain only digits (0-9)"),
            CipherError::ParseError => write!(f, "Failed to parse PIN as a number"),
            CipherError::MissingLaneShift(l) => write!(f, "Missing lane shift for lane {:?}", l),
            CipherError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for CipherError {}

/// Public API: returns the polyalphabetic (per-lane) cipher result or an error.
pub fn poly_cipher(password: &str, pin: &str) -> Result<String, CipherError> {
    let shifts = shift_key(pin)?;
    let key_map = encryption_key(&shifts)?;
    let garbled = garble_password(password, &key_map);
    Ok(garbled)
}

/// Convert the 5-digit pin into lane shifts (A..D).
fn shift_key(pin: &str) -> Result<HashMap<Lane, usize>, CipherError> {
    // Validate pin
    if pin.len() != 5 {
        return Err(CipherError::InvalidPinLength);
    }
    if !pin.chars().all(|c| c.is_ascii_digit()) {
        return Err(CipherError::NonDigitInPin);
    }

    // parse digits safely
    let digits: Vec<u32> = pin
        .chars()
        .map(|c| c.to_digit(10).ok_or(CipherError::NonDigitInPin))
        .collect::<Result<_, _>>()?;

    let a = digits[0] * 10 + digits[1];
    let b = digits[1] * 10 + digits[2];
    let c = digits[2] * 10 + digits[3];
    let d = digits[3] * 10 + digits[4];

    // square the numeric PIN (use wide integer to avoid overflow)
    let pin_num = pin.parse::<u64>().map_err(|_| CipherError::ParseError)? as u128;
    let pin_sqr = pin_num * pin_num;

    // get last 4 digits (least-significant first), fallback to '0' if missing
    let pin_sqr_str = pin_sqr.to_string();
    let mut rev = pin_sqr_str.chars().rev();
    let a_add = rev.next().unwrap_or('0').to_digit(10).ok_or(CipherError::Internal("a_add"))? as usize;
    let b_add = rev.next().unwrap_or('0').to_digit(10).ok_or(CipherError::Internal("b_add"))? as usize;
    let c_add = rev.next().unwrap_or('0').to_digit(10).ok_or(CipherError::Internal("c_add"))? as usize;
    let d_add = rev.next().unwrap_or('0').to_digit(10).ok_or(CipherError::Internal("d_add"))? as usize;

    let mut keys = HashMap::new();
    keys.insert(Lane::A, (a as usize) + a_add);
    keys.insert(Lane::B, (b as usize) + b_add);
    keys.insert(Lane::C, (c as usize) + c_add);
    keys.insert(Lane::D, (d as usize) + d_add);

    Ok(keys)
}

/// Build per-lane substitution maps (char -> char) from the shifts.
fn encryption_key(shifts: &HashMap<Lane, usize>) -> Result<HashMap<Lane, HashMap<char, char>>, CipherError> {
    let n = CHARACTERS.len();
    let lanes = [Lane::A, Lane::B, Lane::C, Lane::D];

    let mut letter_key: HashMap<Lane, HashMap<char, char>> = HashMap::new();

    for &lane in &lanes {
        // get shift for lane, return typed error if missing
        let shift = *shifts.get(&lane).ok_or(CipherError::MissingLaneShift(lane))?;
        let shift = shift % n; // rotate_left handles bigger values but modulo makes intent clear

        // build rotated vector of characters
        let mut rotated: Vec<char> = CHARACTERS.iter().copied().collect();
        rotated.rotate_left(shift);

        let mut map: HashMap<char, char> = HashMap::with_capacity(n);
        for (i, &orig) in CHARACTERS.iter().enumerate() {
            map.insert(orig, rotated[i]);
        }

        letter_key.insert(lane, map);
    }

    Ok(letter_key)
}

/// Apply lane-mapped substitution to the password and return the transformed string.
fn garble_password(password: &str, key: &HashMap<Lane, HashMap<char, char>>) -> String {
    let mut out = String::with_capacity(password.len());
    for (i, c) in password.chars().enumerate() {
        let lane = Lane::from_index(i);
        let mapped = key
            .get(&lane)
            .and_then(|m| m.get(&c))
            .copied()
            .unwrap_or(c); // if char not in CHARACTERS, leave unchanged
        out.push(mapped);
    }
    out
}

