use std::collections::HashMap;


#[allow(unused_variables)]
pub fn poly_cypher(password: &str, pin: &str) -> () {
    if let Err(error) = (|| -> Result<(), String> {
        let shift_key = shift_key(pin)?; // Unpack the result of shift_key
        let encryption_key = encryption_key(shift_key)?; // Unpack the result of encryption_key
        let garbled_password = garble_password(&password, &encryption_key);
        println!("{:?}", encryption_key);
        Ok(())
    })() {
        eprintln!("Error: {}", error);
    }
    println!("{password}");

    // split the password into array of 4 element arrays
    // iterate over array and and in each 4 element array interate and shift letter
    // by corresponding key
}

fn garble_password(password: &str, key: &HashMap<&str, HashMap<char, char>>) -> String {
    let a_key = *key.get("A").unwrap();

    let mut garbled = password
            match index {
                0 => if let chunk = a_key.get(&chunk[index]) {

                },
                // 1 => chunk = key.get("B").get(chunk),
                // 2 => chunk = key.get("C").get(chunk),
                // 3 => chunk = key.get("D").get(chunk),
            }
        })
        // .collect();
    // println!("{garbled}")
}

fn encryption_key(pin_key: HashMap<&str, u32>) -> Result<HashMap<&str, HashMap<char, char>>, String> {
    let mut letter_key: HashMap<&str, HashMap<char, char>> = HashMap::new();

    let mut a_rotate = CHARACTERS.to_vec();
    a_rotate.rotate_left(*pin_key.get("A").ok_or("Key A can't rotate")? as usize);
    let mut b_rotate = CHARACTERS.to_vec();
    b_rotate.rotate_left(*pin_key.get("B").ok_or("Key B can't rotate")? as usize);
    let mut c_rotate = CHARACTERS.to_vec();
    c_rotate.rotate_left(*pin_key.get("C").ok_or("Key C can't rotate")? as usize);
    let mut d_rotate = CHARACTERS.to_vec();
    d_rotate.rotate_left(*pin_key.get("D").ok_or("Key D can't rotate")? as usize);

    let mut a_key: HashMap<char, char> = HashMap::new();
    let mut b_key: HashMap<char, char> = HashMap::new();
    let mut c_key: HashMap<char, char> = HashMap::new();
    let mut d_key: HashMap<char, char> = HashMap::new();

    for i in 0..CHARACTERS.len() {
        a_key.insert(CHARACTERS[i], a_rotate[i]);
        b_key.insert(CHARACTERS[i], b_rotate[i]);
        c_key.insert(CHARACTERS[i], c_rotate[i]);
        d_key.insert(CHARACTERS[i], b_rotate[i]);
    }

    letter_key.insert("A", a_key);
    letter_key.insert("B", b_key);
    letter_key.insert("C", c_key);
    letter_key.insert("D", d_key);

    Ok(letter_key)
}

fn shift_key(pin: &str) -> Result<HashMap<&str, u32>, String> {
    // guard clause for too short or long pin
    if pin.len() != 5 {
        return Err("PIN must be exactly 5 digits long.".to_string());
    }

    let mut keys: HashMap<&str, u32> = HashMap::new();
    let chars: Vec<char> = pin.chars().collect();

    let a = format!("{}{}", chars[0], chars[1]);
    let b = format!("{}{}", chars[1], chars[2]);
    let c = format!("{}{}", chars[2], chars[3]);
    let d = format!("{}{}", chars[3], chars[4]);
    println!("A: {a} \n B: {b} \n C: {c} \n D: {d}");

    let mut pin_sqr = pin
        .parse::<u32>()
        .map_err(|_| "Failed to parse pin.".to_string())?;
    pin_sqr = pin_sqr * pin_sqr;
    println!("{pin_sqr}");
    let pin_sqr_char: Vec<char> = pin_sqr.to_string().chars().collect();
    let a_add = pin_sqr_char[pin_sqr_char.len() - 1]
        .to_string()
        .parse::<u32>()
        .map_err(|_| "Failed to parse A sqr pin additon".to_string())?;
    let b_add = pin_sqr_char[pin_sqr_char.len() - 2]
        .to_string()
        .parse::<u32>()
        .map_err(|_| "Failed to parse B sqr pin additon".to_string())?;
    let c_add = pin_sqr_char[pin_sqr_char.len() - 3]
        .to_string()
        .parse::<u32>()
        .map_err(|_| "Failed to parse C sqr pin additon".to_string())?;
    let d_add = pin_sqr_char[pin_sqr_char.len() - 4]
        .to_string()
        .parse::<u32>()
        .map_err(|_| "Failed to parse D sqr pin additon".to_string())?;

    println!("a: {a_add} \n b: {b_add}\n c: {c_add} \n d: {d_add}");

    let a_val = a
        .parse::<u32>()
        .map_err(|_| "Failed to parse 'A' value.".to_string())?
        + a_add;
    let b_val = b
        .parse::<u32>()
        .map_err(|_| "Failed to parse 'B' value.".to_string())?
        + b_add;
    let c_val = c
        .parse::<u32>()
        .map_err(|_| "Failed to parse 'C' value.".to_string())?
        + c_add;
    let d_val = d
        .parse::<u32>()
        .map_err(|_| "Failed to parse 'D' value.".to_string())?
        + d_add;

    keys.insert("A", a_val);
    keys.insert("B", b_val);
    keys.insert("C", c_val);
    keys.insert("D", d_val);

    Ok(keys)
}

const CHARACTERS: [char; 94] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '!', '@',
    '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', ']', '{', '}', '\\', '|', ';',
    ':', '\'', '"', ',', '.', '<', '>', '/', '?', '`', '~', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
    'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];
