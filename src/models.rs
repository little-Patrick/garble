use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub site: String,
    pub username: String,
    pub pin: String,
    pub password: String,
    pub garbled: String,
}

