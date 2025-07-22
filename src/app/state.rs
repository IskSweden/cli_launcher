// src/app/state.rs
// Holds input, filtered list, selected index etc.

#[derive(Debug, Default)]
pub struct App {
    pub input: String,
    pub command_input: String,
    pub mode: InputMode,
    pub exit: bool,
}

#[derive(Debug, Default)]
pub enum InputMode {
    #[default]
    Insert,
    Command,
}
