use std::collections::HashMap;

pub fn poly_cypher(password: &str, pin: &str) -> Result<String, String> {
    // Derive per-lane shifts from the PIN and build the substitution key
    let shifts = shift_key(pin)?;
    let key_map = encryption_key(shifts)?;
    let garbled_password = garble_password(password, &key_map);
    Ok(garbled_password)
}

fn garble_password(password: &str, key: &HashMap<&str, HashMap<char, char>>) -> String {
    // Fetch lane maps with helpful messages if the invariant breaks.
    let a_key = key
        .get("A")
        .expect("encryption key missing lane A; this is a programming invariant");
    let b_key = key
        .get("B")
        .expect("encryption key missing lane B; this is a programming invariant");
    let c_key = key
        .get("C")
        .expect("encryption key missing lane C; this is a programming invariant");
    let d_key = key
        .get("D")
        .expect("encryption key missing lane D; this is a programming invariant");

    let mut out = String::with_capacity(password.len());
    for (i, c) in password.chars().enumerate() {
        let mapped = match i % 4 {
            0 => *a_key.get(&c).unwrap_or(&c),
            1 => *b_key.get(&c).unwrap_or(&c),
            2 => *c_key.get(&c).unwrap_or(&c),
            _ => *d_key.get(&c).unwrap_or(&c),
        };
        out.push(mapped);
    }
    out
}

fn encryption_key(pin_key: HashMap<&'static str, usize>) -> Result<HashMap<&'static str, HashMap<char, char>>, String> {
    let mut letter_key: HashMap<&str, HashMap<char, char>> = HashMap::new();

    let mut a_rotate = CHARACTERS.to_vec();
    a_rotate.rotate_left(*pin_key.get("A").ok_or("Missing lane A shift")?);
    let mut b_rotate = CHARACTERS.to_vec();
    b_rotate.rotate_left(*pin_key.get("B").ok_or("Missing lane B shift")?);
    let mut c_rotate = CHARACTERS.to_vec();
    c_rotate.rotate_left(*pin_key.get("C").ok_or("Missing lane C shift")?);
    let mut d_rotate = CHARACTERS.to_vec();
    d_rotate.rotate_left(*pin_key.get("D").ok_or("Missing lane D shift")?);

    let mut a_key: HashMap<char, char> = HashMap::new();
    let mut b_key: HashMap<char, char> = HashMap::new();
    let mut c_key: HashMap<char, char> = HashMap::new();
    let mut d_key: HashMap<char, char> = HashMap::new();

    for i in 0..CHARACTERS.len() {
        a_key.insert(CHARACTERS[i], a_rotate[i]);
        b_key.insert(CHARACTERS[i], b_rotate[i]);
        c_key.insert(CHARACTERS[i], c_rotate[i]);
        d_key.insert(CHARACTERS[i], d_rotate[i]);
    }

    letter_key.insert("A", a_key);
    letter_key.insert("B", b_key);
    letter_key.insert("C", c_key);
    letter_key.insert("D", d_key);

    Ok(letter_key)
}

fn shift_key(pin: &str) -> Result<HashMap<&'static str, usize>, String> {
    // Validate exact length and digit-only PIN.
    if pin.len() != 5 {
        return Err("PIN must be exactly 5 digits long.".to_string());
    }
    if !pin.chars().all(|c| c.is_ascii_digit()) {
        return Err("PIN must contain only digits (0-9).".to_string());
    }

    // Extract digits directly without allocating strings.
    let digits: Vec<u32> = pin
        .chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or_else(|| "Invalid digit in PIN".to_string())
        })
        .collect::<Result<_, _>>()?;

    let a = digits[0] * 10 + digits[1];
    let b = digits[1] * 10 + digits[2];
    let c = digits[2] * 10 + digits[3];
    let d = digits[3] * 10 + digits[4];

    // Square in a wide integer to avoid overflow for 5-digit PINs.
    let pin_num = pin
        .parse::<u64>()
        .map_err(|_| "Failed to parse PIN".to_string())? as u128;
    let pin_sqr = pin_num * pin_num;

    // Safely take last four digits of the square, padding with zeros as needed.
    let pin_sqr_str = pin_sqr.to_string();
    let mut it = pin_sqr_str.chars().rev();
    let a_add = it
        .next()
        .unwrap_or('0')
        .to_digit(10)
        .ok_or_else(|| "Failed to read A square digit".to_string())? as usize;
    let b_add = it
        .next()
        .unwrap_or('0')
        .to_digit(10)
        .ok_or_else(|| "Failed to read B square digit".to_string())? as usize;
    let c_add = it
        .next()
        .unwrap_or('0')
        .to_digit(10)
        .ok_or_else(|| "Failed to read C square digit".to_string())? as usize;
    let d_add = it
        .next()
        .unwrap_or('0')
        .to_digit(10)
        .ok_or_else(|| "Failed to read D square digit".to_string())? as usize;

    let mut keys: HashMap<&'static str, usize> = HashMap::new();
    keys.insert("A", (a as usize) + a_add);
    keys.insert("B", (b as usize) + b_add);
    keys.insert("C", (c as usize) + c_add);
    keys.insert("D", (d as usize) + d_add);

    Ok(keys)
}

const CHARACTERS: [char; 94] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '@',
    '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', ']', '{', '}', '\\', '|', ';',
    ':', '\'', '"', ',', '.', '<', '>', '/', '?', '`', '~', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

