use std::fs;
use std::path::Path;
use crate::models::Login;
use serde_json;

const FILE_PATH: &str = "logins.json";

pub fn load_logins() -> Vec<Login> {
    if Path::new(FILE_PATH).exists() {
        let data = fs::read_to_string(FILE_PATH).unwrap_or_else(|_| "[]".to_string());
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

pub fn save_logins(logins: &Vec<Login>) {
    let data = serde_json::to_string_pretty(logins).expect("Failed to serialize");
    fs::write(FILE_PATH, data).expect("Unable to write file");
}
