use crate::models::Login;
use crate::garble::poly_cypher;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Page {
    Home,
    Add,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputField {
    Site,
    Username,
    Pin,
    Password,
}

pub struct App {
    pub current_page: Page,
    pub logins: Vec<Login>,
    pub site_input: String,
    pub username_input: String,
    pub pin_input: String,
    pub password_input: String,
    pub active_field: InputField,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_page: Page::Home,
            logins: Vec::new(),
            site_input: String::new(),
            username_input: String::new(),
            pin_input: String::new(),
            password_input: String::new(),
            active_field: InputField::Site,
        }
    }

    pub fn clear_form(&mut self) {
        self.site_input.clear();
        self.username_input.clear();
        self.pin_input.clear();
        self.password_input.clear();
        self.active_field = InputField::Site;
    }

    pub fn add_login(&mut self) {
       // Error handling for Pin login or any other field 
       let password = &self.password_input;
       let pin = &self.pin_input;
       let garbled = match poly_cypher(password, pin) {
        Ok(garbled) => garbled,
        Err(_) => return 
       };

       let login = Login {
           site: self.site_input.clone(),
           username: self.username_input.clone(),
           pin: self.pin_input.clone(),
           password: self.password_input.clone(),
           garbled: garbled,
       };

       self.logins.push(login);
    }
}
