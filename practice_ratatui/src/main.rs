//! Entry point and event loop for the Day Planner TUI app.
//! Handles terminal setup, navigation, and delegates UI rendering.

// --- Imports ---
use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen},
};
use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::layout::{Layout, Constraint, Direction};
use std::io;
mod state;
use state::{AppState, Page, Task};
mod ui;
#[allow(unused_imports)]
use ui::*;

/// Main entry point. Sets up the terminal, runs the event loop, and handles cleanup.
/// 
/// # Structure
/// - Sets up Crossterm and Ratatui terminal.
/// - Initializes the app state (see state.rs).
/// - Runs the main event loop:
///     - Draws the UI (delegates to ui.rs).
///     - Handles keyboard input for navigation and actions.
///     - Handles input mode for adding tasks.
/// - Cleans up the terminal on exit.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- Terminal setup ---
    // Enable raw mode and switch to alternate screen for TUI experience.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // --- App state initialization ---
    // This struct holds all app data: tasks, navigation, input, etc.
    let mut state = AppState {
        items: vec![
            Task { description: "First item".to_string(), done: false },
            Task { description: "Second item".to_string(), done: false },
            Task { description: "Third item".to_string(), done: false },
        ],
        selected: 0,
        page: Page::Main,
        input: String::new(),
        input_mode: false,
        // --- Form state ---
        show_form: false,
        form_description: String::new(),
        form_done: false,
        form_field: state::FormField::Description,
    };

    // --- Main event loop ---
    // This loop runs until the user presses 'q'.
    // It draws the UI and handles all keyboard input.
    loop {
        // --- Draw the UI for the current state ---
        terminal.draw(|frame| {
            let size = frame.size();
            // Layout: header, main, input, status
            // We use Ratatui's Layout to split the screen into regions.
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),   // Header
                    Constraint::Min(5),      // Main
                    Constraint::Length(3),   // Input
                    Constraint::Length(2),   // Status
                ].as_ref())
                .split(size);

            // Main area split horizontally
            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(25),  // Info
                    Constraint::Percentage(50),  // Task List
                    Constraint::Percentage(25),  // Details
                ].as_ref())
                .split(vertical_chunks[1]);

            // --- Render header ---
            // The header is always shown at the top.
            let header = draw_header();
            frame.render_widget(header, vertical_chunks[0]);

            // --- Render main content based on current page ---
            // Each page (Main, Planner, Notes, Help) has its own UI.
            match state.page {
                Page::Main => {
                    let home = Paragraph::new("Welcome to your Day Planner!\n\nPress 1: Home | 2: Planner | 3: Notes | ?: Help")
                        .block(Block::default().title("Home").borders(Borders::ALL));
                    frame.render_widget(home, main_chunks[0]);
                    let empty = Block::default().title("").borders(Borders::ALL);
                    frame.render_widget(empty, main_chunks[1]);
                }
                Page::Planner => {
                    let left_panel = draw_left_panel();
                    frame.render_widget(left_panel, main_chunks[0]);
                    let (right_panel, mut list_state) = draw_right_panel(&state);
                    frame.render_stateful_widget(right_panel, main_chunks[1], &mut list_state);
                    let details_panel = draw_task_details(&state);
                    frame.render_widget(details_panel, main_chunks[2]);
                }
                Page::Notes => {
                    let notes = draw_notes();
                    frame.render_widget(notes, main_chunks[0]);
                    let empty = Block::default().title("").borders(Borders::ALL);
                    frame.render_widget(empty, main_chunks[1]);
                }
                Page::Help => {
                    let help = draw_help();
                    frame.render_widget(help, main_chunks[0]);
                    let empty = Block::default().title("").borders(Borders::ALL);
                    frame.render_widget(empty, main_chunks[1]);
                }
            }

            // --- Render form popup if open ---
            if state.show_form {
                let popup_area = centered_rect(60, 30, size);
                let form = ui::draw_form(&state);
                frame.render_widget(form, popup_area);
            }

            // --- Render input area (only in Planner input mode) ---
            // Only show the input box when adding a task in Planner.
            if state.input_mode && state.page == Page::Planner {
                let input_panel = Paragraph::new(state.input.as_ref())
                    .block(Block::default().title("Input").borders(Borders::ALL));
                frame.render_widget(input_panel, vertical_chunks[2]);
            } else {
                let empty = Block::default().borders(Borders::ALL);
                frame.render_widget(empty, vertical_chunks[2]);
            }

            // --- Render status bar ---
            // Shows current mode (Normal/Input) and hints.
            let status = draw_status(&state);
            frame.render_widget(status, vertical_chunks[3]);
        })?;

        // --- Handle input events ---
        // Keyboard input is handled here. We distinguish between normal and input mode.
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                // --- Form input handling ---
                if state.show_form {
                    use state::FormField;
                    match key_event.code {
                        KeyCode::Tab | KeyCode::Down => {
                            state.form_field = match state.form_field {
                                FormField::Description => FormField::Done,
                                FormField::Done => FormField::Description,
                            };
                        }
                        KeyCode::BackTab | KeyCode::Up => {
                            state.form_field = match state.form_field {
                                FormField::Description => FormField::Done,
                                FormField::Done => FormField::Description,
                            };
                        }
                        KeyCode::Char(' ') => {
                            if state.form_field == FormField::Done {
                                state.form_done = !state.form_done;
                            } else {
                                state.form_description.push(' ');
                            }
                        }
                        KeyCode::Char(c) => {
                            if state.form_field == FormField::Description {
                                state.form_description.push(c);
                            }
                        }
                        KeyCode::Backspace => {
                            if state.form_field == FormField::Description {
                                state.form_description.pop();
                            }
                        }
                        KeyCode::Enter => {
                            // Submit form
                            if !state.form_description.trim().is_empty() {
                                state.items.push(Task {
                                    description: state.form_description.trim().to_string(),
                                    done: state.form_done,
                                });
                                state.selected = state.items.len() - 1;
                            }
                            state.show_form = false;
                        }
                        KeyCode::Esc => {
                            // Cancel form
                            state.show_form = false;
                        }
                        _ => {}
                    }
                    continue;
                }
                // Input mode (for adding tasks in Planner)
                // Only active when adding a new task in Planner.
                if state.input_mode && state.page == Page::Planner {
                    match key_event.code {
                        KeyCode::Char(c) => state.input.push(c),
                        KeyCode::Backspace => { state.input.pop(); },
                        KeyCode::Enter => {
                            if !state.input.is_empty() {
                                state.items.push(Task { description: state.input.clone(), done: false });
                                state.input.clear();
                                state.selected = state.items.len() - 1;
                            }
                            state.input_mode = false;
                        }
                        KeyCode::Esc => {
                            state.input_mode = false;
                        }
                        _ => {}
                    }
                    continue;
                }
                // Normal mode (navigation, actions)
                // Handles navigation, page switching, saving/loading, etc.
                if key_event.code == KeyCode::Char('n') {
                    state.show_form = true;
                    state.form_description.clear();
                    state.form_done = false;
                    state.form_field = state::FormField::Description;
                    continue;
                }
                if state.handle_event(key_event.code) {
                    break;
                }
            }
        }
    }
    // --- Cleanup terminal ---
    // Restore terminal to original state before exiting.
    disable_raw_mode()?;
    execute!(std::io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

// Helper function for centered popup
fn centered_rect(percent_x: u16, percent_y: u16, r: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let popup_layout = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            ratatui::layout::Constraint::Percentage((100 - percent_y) / 2),
            ratatui::layout::Constraint::Percentage(percent_y),
            ratatui::layout::Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    let vertical = popup_layout[1];
    let horizontal_layout = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Percentage((100 - percent_x) / 2),
            ratatui::layout::Constraint::Percentage(percent_x),
            ratatui::layout::Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical);
    horizontal_layout[1]
}
