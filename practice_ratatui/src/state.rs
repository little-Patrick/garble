//! App state and persistence logic for the Day Planner TUI app.
//! Defines the AppState struct, Page enum, and save/load methods.

use std::fs::File;
use std::io::{self, Write, BufRead, BufReader};
use crossterm::event::KeyCode;
use serde::{Serialize, Deserialize};
use serde_json;

/// Represents a single planner task, with a description and completion flag.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub done: bool,
}

/// Holds all mutable state for the app (tasks, navigation, input, etc).
pub struct AppState {
    pub items: Vec<Task>,      // List of planner tasks
    pub selected: usize,         // Index of selected task
    pub page: Page,              // Current page (Main, Planner, Notes, Help)
    pub input: String,           // Input buffer for adding tasks
    pub input_mode: bool,        // Whether we're in input mode
    // --- Form state ---
    pub show_form: bool,         // Is the form open?
    pub form_description: String,
    pub form_done: bool,
    pub form_field: FormField,   // Which field is focused?
}

/// Enum for all possible pages/screens in the app.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Main,       // Home page
    Planner,    // Task planner page
    Notes,      // Notes page
    Help,       // Help/about page
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FormField {
    Description,
    Done,
}

impl AppState {
    /// Save all tasks to a JSON file.
    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        let file = File::create(path)?;
        serde_json::to_writer_pretty(file, &self.items)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    /// Load all tasks from a JSON file.
    /// Resets selection if needed.
    pub fn load_from_file(&mut self, path: &str) -> io::Result<()> {
        let file = File::open(path)?;
        let items: Vec<Task> = serde_json::from_reader(file)
            .unwrap_or_default();
        self.items = items;
        // Reset selection if needed
        if self.selected >= self.items.len() && !self.items.is_empty() {
            self.selected = self.items.len() - 1;
        } else if self.items.is_empty() {
            self.selected = 0;
        }
        Ok(())
    }

    /// Handles a key event in normal mode (not input mode).
    /// Returns true if the app should quit, false otherwise.
    pub fn handle_event(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') => return true, // Quit
            KeyCode::Char('1') => self.page = Page::Main,
            KeyCode::Char('2') => self.page = Page::Planner,
            KeyCode::Char('3') => self.page = Page::Notes,
            KeyCode::Char('?') => self.page = Page::Help,
            KeyCode::Char('j') => {
                if self.page == Page::Planner && self.selected < self.items.len().saturating_sub(1) {
                    self.selected += 1;
                }
            }
            KeyCode::Char('k') => {
                if self.page == Page::Planner && self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Char('i') => {
                if self.page == Page::Planner {
                    self.input_mode = true;
                }
            }
            KeyCode::Char('d') => {
                if self.page == Page::Planner && !self.items.is_empty() {
                    self.items.remove(self.selected);
                    if self.selected >= self.items.len() && !self.items.is_empty() {
                        self.selected = self.items.len() - 1;
                    } else if self.items.is_empty() {
                        self.selected = 0;
                    }
                }
            }
            KeyCode::Char(' ') => {
                if self.page == Page::Planner && !self.items.is_empty() {
                    let task = &mut self.items[self.selected];
                    task.done = !task.done;
                }
            }
            KeyCode::Char('s') => {
                let _ = self.save_to_file("tasks.json");
            }
            KeyCode::Char('l') => {
                let _ = self.load_from_file("tasks.json");
            }
            _ => {}
        }
        false
    }
}
