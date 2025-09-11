use crossterm::event::{Event, KeyCode};
use crate::app::{App, Fields};
use crate::models::Login;
use crate::storage::save_login;
use crate::garble::poly::poly_cypher; // direct path since main declares mod garble

pub fn handle_input(event: Event, app: &mut App) -> bool {
    if let Event::Key(key) = event {
        match key.code {
            KeyCode::Esc => return true, // quit app
            KeyCode::Enter => {
                match app.input_field {
                    Fields::Site => {
                        app.current_site = app.input.trim().to_string();
                        app.input.clear();
                        app.input_field = Fields::Username;
                    }
                    Fields::Username => {
                        app.current_username = app.input.trim().to_string();
                        app.input.clear();
                        app.input_field = Fields::Pin;
                    }
                    Fields::Pin => {
                        app.current_pin = app.input.trim().to_string();
                        app.input.clear();
                        app.input_field = Fields::Password;
                    }
                    Fields::Password => {
                        app.current_password = app.input.clone();
                        let pin = app.current_pin.clone();
                        let password = app.current_password.clone();
                        // Validate pin early.
                        if pin.len() != 5 || !pin.chars().all(|c| c.is_ascii_digit()) {
                            app.error = Some("PIN must be exactly 5 digits".to_string());
                        } else {
                            match poly_cypher(&password, &pin) {
                                Ok(garbled) => {
                                    let login = Login {
                                        site: std::mem::take(&mut app.current_site),
                                        username: std::mem::take(&mut app.current_username),
                                        pin: pin.clone(),
                                        password: password.clone(),
                                        garbled,
                                    };
                                    app.logins.push(login);
                                    if let Err(e) = std::panic::catch_unwind(|| save_login(&app.logins)) {
                                        app.error = Some(format!("Failed to save: {:?}", e));
                                    } else {
                                        app.error = None;
                                    }
                                }
                                Err(err) => {
                                    app.error = Some(err);
                                }
                            }
                        }
                        // Reset draft (note: current_* site already moved; others cleared explicitly)
                        app.current_username.clear();
                        app.current_pin.clear();
                        app.current_password.clear();
                        app.input.clear();
                        app.input_field = Fields::Site;
                    }
                }
            }
            KeyCode::Char(c) => {
                app.input.push(c);
            }
            KeyCode::Backspace => {
                app.input.pop();
            }
            _ => {}
        }
    }
    false
}

