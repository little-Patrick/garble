use std::collections::HashMap;



pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum Fields {
    Site,
    Username,
    Pin,
    Password,
}

pub struct App {
    pub site_input: String,              // the currently being edited json key.
    pub username_input: String,              // the currently being edited json key.
    pub pin_input: String,            // the currently being edited json value.
    pub password_input: String,            // the currently being edited json value.
    pub garbled: String,
    pub pairs: HashMap<String, String>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<Fields>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
}

impl App {
    pub fn new() -> App {
        App {
            site_input: String::new(),
            username_input: String::new(),
            pin_input: String::new(),
            password_input: String::new(),
            garbled: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs.insert("site".to_string(), self.site_input.clone());
        self.pairs.insert("username".to_string(), self.username_input.clone());
        self.pairs.insert("pin".to_string(), self.pin_input.clone());
        self.pairs.insert("password".to_string(), self.password_input.clone());
        self.pairs.insert("garbled".to_string(), self.site_input.clone());

        self.site_input = String::new();
        self.username_input = String::new();
        self.pin_input = String::new();
        self.password_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                Fields::Site => self.currently_editing = Some(Fields::Username),
                Fields::Username => self.currently_editing = Some(Fields::Pin),
                Fields::Pin => self.currently_editing = Some(Fields::Password),
                Fields::Password => self.currently_editing = Some(Fields::Site),
            };
        } else {
            self.currently_editing = Some(Fields::Site);
        }
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{output}");
        Ok(())
    }
}
