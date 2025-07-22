// src/app/state.rs
// Holds input, filtered list, selected index etc.

#[derive(Debug, Default)]
pub struct App {
    pub input: String,
    pub exit: bool,
}