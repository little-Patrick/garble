// use std::io;
mod garble;
use garble::poly_cypher;

fn main() {
    // println!("Please enter username...");
    // let mut user_name = String::new();
    // io::stdin().read_line(&mut user_name)
    //     .expect("Failed to read username");
    //
    // println!("Please enter 5 digit pin...");
    // let mut pin = String::new();
    // io::stdin().read_line(&mut pin)
    //     .expect("Failed to read password");
    //
    // println!("Please enter password...");
    // let mut password = String::new();
    // io::stdin().read_line(&mut password)
    //     .expect("Failed to read password");
    //
    // // Remove trailing newline characters
    // let user_name = user_name.trim();
    // let password = password.trim();
    // let pin = pin.trim();

    let user_name = "User Name";
    let pin = "12345";
    let password = "Password";
    // let garble_password = poly_cypher(&password, &pin);
    poly_cypher(&password, &pin);

    
    println!("Username: {}", user_name);
    println!("OG Password: {}", password);
    println!("{:?}", pin)
    // println!("Password: {}", garble_password);
}

