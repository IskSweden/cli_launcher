// src/app/state.rs
// Holds input, filtered list, selected index etc.

#[derive(Debug, Default)]
pub struct App {
    input: String,
    wants_to_quit: bool,
}