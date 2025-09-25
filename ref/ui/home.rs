use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, Page, InputField};

pub fn draw_ui(frame: &mut Frame, app: &App) {
    match app.current_page {
        Page::Home => draw_home(frame),
        Page::Add => draw_add(frame, &app),
    }
    
    fn draw_home(frame: &mut Frame) {
        let size = frame.area();
        let block = Block::default()
            .title("Home page")
            .borders(Borders::ALL);
        frame.render_widget(block, size);
    }

    fn draw_add(frame: &mut Frame, app: &App) {
        let size = frame.area();

        // Split Screen | Form | Generated Password
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(size);

        // Left side: Form
        let form_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Site form
            Constraint::Length(3), // Username form
            Constraint::Length(3), // Pin form
            Constraint::Length(3), // Password form
                                   //
                                   // TODO: Buttons?
        ])
        .split(chunks[0]);

        let site_input = &app.site_input;
        let site = Paragraph::new(site_input.to_string())
            .block(Block::default().title("Website").borders(Borders::ALL));
        frame.render_widget(site, form_chunks[0]);

        let username_input = &app.username_input;
        let site = Paragraph::new(username_input.to_string())
            .block(Block::default().title("Username").borders(Borders::ALL));
        frame.render_widget(site, form_chunks[1]);

        let pin_input = &app.pin_input;
        let site = Paragraph::new(pin_input.to_string())
            .block(Block::default().title("Pin").borders(Borders::ALL));
        frame.render_widget(site, form_chunks[2]);

        let password_input = &app.password_input;
        let site = Paragraph::new(password_input.to_string())
            .block(Block::default().title("Password").borders(Borders::ALL));
        frame.render_widget(site, form_chunks[3]);

        // Right side: Logins
        let logins: Vec<String> = app.logins
            .iter()
            .map(|i| 
                format!("
                Website: {}\n
                Username: {}\n
                Pin: {}\n
                Password: {}\n
                Garbled: {}\n", 
                i.site, i.username, i.pin, i.password, i.garbled))
            .collect();
        let display_logins = Paragraph::new(logins.join("\n"))
            .block(Block::default().title("Logins").borders(Borders::ALL));

        frame.render_widget(display_logins, chunks[1]);
    }

}
