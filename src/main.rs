// src/main.rs
// Entry point + app loop

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

mod app;
mod launcher;
use crate::app::state::{App, InputMode};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}


impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn draw(&mut self, frame: &mut Frame) {
        
        let greeting = Paragraph::new(&*self.input);

        frame.render_widget(greeting, frame.area());
    }


    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.mode {
            InputMode::Insert => {
                match key_event.code {

                    KeyCode::Char(':') => {
                        self.mode = InputMode::Command;
                        self.command_input.clear();
                    },

                    KeyCode::Char(c) => { self.input.push(c); },

                    KeyCode::Backspace => { self.input.pop(); },

                    _ => {}
                }
            }
            InputMode::Command => {
                match key_event.code {

                    KeyCode::Char(c) => { self.command_input.push(c); },
                    
                    KeyCode::Enter => {
                        if self.command_input == "q" || self.command_input == "quit" {
                            self.exit()
                        }
                    },

                    KeyCode::Esc => { self.mode = InputMode::Insert },

                    KeyCode::Backspace => { self.command_input.pop(); },

                    _ => {}

                }

                    
            }
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    
}

