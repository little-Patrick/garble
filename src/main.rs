use std::io;
mod garble;
use garble::poly_cypher;

fn main() {
    // CLI input
    println!("Please enter username...");
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name)
        .expect("Failed to read username");

    println!("Please enter 5 digit pin...");
    let mut pin = String::new();
    io::stdin().read_line(&mut pin)
        .expect("Failed to read pin");

    println!("Please enter password...");
    let mut password = String::new();
    io::stdin().read_line(&mut password)
        .expect("Failed to read password");

    // Remove trailing newline characters
    let user_name = user_name.trim();
    let password = password.trim();
    let pin = pin.trim();

    // Call poly_cypher and handle the result
    match poly_cypher(password, pin) {
        Ok(garbled_password) => {
            println!("Username: {}", user_name);
            println!("Original Password: {}", password);
            println!("Garbled Password: {}", garbled_password);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
}

