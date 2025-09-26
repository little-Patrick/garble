//! UI rendering functions for the Day Planner TUI app.
//! All UI widgets and layout logic are defined here.

use ratatui::widgets::{Block, Borders, Paragraph, List, ListItem, ListState};
use ratatui::style::{Style, Modifier, Color};
use crate::{AppState};
use crate::state::Task;

/// Draws the static Notes page (notepad placeholder).
pub fn draw_notes<'a>() -> Paragraph<'a> {
    Paragraph::new("This is your notepad. (Static text for now)")
        .block(Block::default().title("Notes").borders(Borders::ALL))
}

/// Draws the static Help page with keybindings and navigation info.
pub fn draw_help<'a>() -> Paragraph<'a> {
    let help_text = "Help Page\n\n1: Home\n2: Planner\n3: Notes\n?: Help\nq: Quit\ni: Add\nd: Delete\ns: Save\nl: Load\nj/k: Move (Planner only)";
    Paragraph::new(help_text)
        .block(Block::default().title("Help").borders(Borders::ALL))
}

/// Draws the header at the top of the app.
pub fn draw_header<'a>() -> Paragraph<'a> {
    Paragraph::new("Day Planner App - Press 'i' to add, 'q' to quit, 'h' for help")
        .block(Block::default().borders(Borders::ALL))
}

/// Draws the left info panel on the Planner page.
pub fn draw_left_panel<'a>() -> Paragraph<'a> {
    Paragraph::new("Welcome to your Day Planner!\nPress 'i' to add a task.")
        .block(Block::default().title("Info").borders(Borders::ALL))
}

/// Draws the right panel (task list) on the Planner page.
pub fn draw_right_panel<'a>(state: &'a AppState) -> (List<'a>, ListState) {
    let items: Vec<ListItem> = state.items
        .iter()
        .map(|task| {
            let prefix = if task.done { "[x] " } else { "[ ] " };
            let style = if task.done {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default()
            };
            ListItem::new(format!("{}{}", prefix, task.description)).style(style)
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(state.selected));

    let list = List::new(items)
        .block(Block::default().title("Right").borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    (list, list_state)
}

/// Draws the status bar at the bottom, showing current mode.
pub fn draw_status<'a>(state: &AppState) -> Paragraph<'a> {
    let msg = if state.input_mode {
        "Input mode: type and press Enter to add, Esc to cancel"
    } else {
        "Normal mode: j/k to move, i to add, h for help"
    };
    Paragraph::new(msg)
        .block(Block::default().borders(Borders::ALL))
}

/// Draws the details panel for the selected task.
pub fn draw_task_details<'a>(state: &'a AppState) -> Paragraph<'a> {
    let details = if let Some(task) = state.items.get(state.selected) {
        format!(
            "Task Details:\n\nDescription: {}\nStatus: {}",
            task.description,
            if task.done { "Completed" } else { "Pending" }
        )
    } else {
        "No task selected.".to_string()
    };
    Paragraph::new(details)
        .block(Block::default().title("Details").borders(Borders::ALL))
}

/// Draws the new task form as a popup overlay.
pub fn draw_form<'a>(state: &'a crate::AppState) -> Paragraph<'a> {
    use crate::state::FormField;
    let desc_marker = if state.form_field == FormField::Description { ">>" } else { "  " };
    let done_marker = if state.form_field == FormField::Done { ">>" } else { "  " };
    let done_box = if state.form_done { "[x]" } else { "[ ]" };
    let text = format!(
        "{} Description: {}\n{} Done: {}",
        desc_marker, state.form_description, done_marker, done_box
    );
    Paragraph::new(text)
        .block(Block::default().title("New Task").borders(Borders::ALL))
}
