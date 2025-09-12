use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, Page, InputField};

pub fn draw_ui(frame: &mut Frame, app: &App) {
    match app.current_page {
        Page::Home => draw_home(frame),
        Page::Add => draw_add(frame),
    }
    
    fn draw_home(frame: &mut Frame) {
        let size = frame.area();
        let block = Block::default()
            .title("Home page")
            .borders(Borders::ALL);
        frame.render_widget(block, size);
    }

    fn draw_add(frame: &mut Frame) {
        
    }

}
