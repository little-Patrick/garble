use crossterm::event::{Event, KeyCode};
use crate::app::{App, Page, InputField};
use crate::storage::save_logins;

pub fn handle_input(event: Event, app: &mut App) -> bool {
    if let Event::Key(key) = event {
        match app.current_page {
            Page::Home => match key.code {
                KeyCode::Char('a') => app.current_page = Page::Add,
                KeyCode::Esc => return true,
                _ => {},
            },
            Page::Add => match key.code {
                KeyCode::Esc => app.current_page = Page::Home,
                KeyCode::Tab => app.active_field = match app.active_field {
                    InputField::Site => InputField::Username,
                    InputField::Username => InputField::Pin,
                    InputField::Pin => InputField::Password,
                    InputField::Password => InputField::Site,
                    _ => InputField::Site,
                },
                KeyCode::Backspace => {
                    match app.active_field {
                        InputField::Site => { app.site_input.pop(); }
                        InputField::Username => { app.username_input.pop(); }
                        InputField::Pin => { app.pin_input.pop(); }
                        InputField::Password => { app.password_input.pop(); }
                    }
                },
                KeyCode::Char(c) => {
                    match app.active_field {
                        InputField::Site => { app.site_input.push(c); }
                        InputField::Username => { app.username_input.push(c); }
                        InputField::Pin => { app.pin_input.push(c); }
                        InputField::Password => { app.password_input.push(c); }
                    }
                }
                KeyCode::Char('c') => {
                    app.clear_form();
                }
                KeyCode::Enter => {
                    app.add_login();
                    save_logins(&app.logins);
                }
                _ => {}

            }
        }
    }
    false
}

