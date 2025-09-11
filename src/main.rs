mod app;
mod ui;
mod input;
mod storage;
mod models;
mod garble; // add crypto module for binary use

use app::App;
use input::handle_input;
use ui::draw_ui;
use storage::load_logins;

use crossterm::{
    event::{self, Event, DisableMouseCapture, EnableMouseCapture},
    execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, Terminal};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load saved books
    let books = load_logins();
    let mut app = App::new(books);

    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| draw_ui(f, app))?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if handle_input(Event::Key(key), app) {
                    return Ok(());
                }
            }
        }
    }
}

