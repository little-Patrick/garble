use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use crate::app::{App, Fields};

pub fn draw_ui(frame: &mut Frame, app: &App) {
    let size = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(size);

    let input = Paragraph::new(app.input.as_str())
        .block(Block::default().borders(Borders::ALL).title(match app.input_field {
            Fields::Site => "Enter Site / App Name",
            Fields::Username => "Enter Username",
            Fields::Pin => "Enter 5 Digit PIN",
            Fields::Password => "Enter Password",
        }));

    frame.render_widget(input, chunks[0]);

    // Draft summary and stored logins
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!(
        "Draft -> Site: {} | Username: {} | PIN: {} | Password: {}",
        app.current_site,
        app.current_username,
        app.current_pin,
        if app.current_password.is_empty() { "<hidden>" } else { "<set>" }
    ));
    if let Some(err) = &app.error {
        lines.push(format!("Error: {}", err));
    }
    if !app.logins.is_empty() {
        lines.push("".to_string());
        for b in &app.logins {
            lines.push(format!(
                "Site: {}\n Username: {}\n PIN: {}\n Original: {}\n Garbled: {}\n",
                b.site, b.username, b.pin, b.password, b.garbled
            ));
        }
    }
    let logins_display = Paragraph::new(lines.join("\n"))
        .block(Block::default().borders(Borders::ALL).title("Logins"));
    frame.render_widget(logins_display, chunks[1]);
}

