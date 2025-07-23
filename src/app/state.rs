// src/app/state.rs
// Holds input, filtered list, selected index etc.


use crate::launcher::appentry::AppEntry;
use crate::launcher::discover::iter_path_bins;

#[derive(Debug)]
pub struct App {
    pub input: String,
    pub command_input: String,
    pub mode: InputMode,
    pub exit: bool,


    pub all_apps: Vec<AppEntry>
}

#[derive(Debug, Default)]
pub enum InputMode {
    #[default]
    Insert,
    Command,
}


impl Default for App {
    fn default() -> Self {
        let all_apps = iter_path_bins(); // Scanning, change later

        println!("Discovered {} executables", all_apps.len()); // Debug

        App {
            input: String::new(),
            command_input:String::new(),
            mode: InputMode::Insert,
            exit: false,
            all_apps,
        }
    }
}