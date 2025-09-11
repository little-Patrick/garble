use crate::models::Login;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fields {
    Site,
    Username,
    Pin,
    Password,
}

pub struct App {
    pub logins: Vec<Login>,
    // Current text buffer for the active field
    pub input: String,
    // Which field is currently active
    pub input_field: Fields,
    // Draft values collected across fields before final submission
    pub current_site: String,
    pub current_username: String,
    pub current_pin: String,
    pub current_password: String,
    // Optional error message to show in UI
    pub error: Option<String>,
}

impl App {
    pub fn new(logins: Vec<Login>) -> Self {
        Self {
            logins,
            input: String::new(),
            input_field: Fields::Site,
            current_site: String::new(),
            current_username: String::new(),
            current_pin: String::new(),
            current_password: String::new(),
            error: None,
        }
    }
    pub fn clear_draft(&mut self) {
        self.current_site.clear();
        self.current_username.clear();
        self.current_pin.clear();
        self.current_password.clear();
        self.input.clear();
        self.input_field = Fields::Site;
    }
}

