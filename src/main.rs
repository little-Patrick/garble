// main.rs - Entry point of our application
use std::io;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

mod app;
// mod storage;

use app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal for TUI mode
    // Raw mode allows us to capture individual key presses
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    
    // Create terminal backend using crossterm
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create our app instance and run it
    let mut app = App::new();
    let result = app.run(&mut terminal);

    // Cleanup: restore terminal to normal mode
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Return any errors that occurred
    result
}
