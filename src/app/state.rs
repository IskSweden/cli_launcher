// src/app/state.rs
// Holds input, filtered list, selected index etc.

use crate::launcher::appentry::AppEntry;
use crate::launcher::discover::{discover_desktop_entries, iter_path_bins};

#[derive(Debug)]
pub struct App {
    pub input: String,
    pub command_input: String,
    pub mode: InputMode,
    pub exit: bool,

    pub all_apps: Vec<AppEntry>,
    pub selected_index: usize,

    pub filtered_apps: Vec<AppEntry>,
}

#[derive(Debug, Default)]
pub enum InputMode {
    #[default]
    Insert,
    Command,
}

impl Default for App {
    fn default() -> Self {
        let mut all_apps = iter_path_bins();
        let desktop_apps = discover_desktop_entries();
        let filtered_apps = all_apps.clone();

        all_apps.extend(desktop_apps);

        App {
            input: String::new(),
            command_input: String::new(),
            mode: InputMode::Insert,
            exit: false,
            all_apps,
            selected_index: 0,
            filtered_apps,
        }
    }
}
