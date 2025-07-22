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
use crate::app::state::App;

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
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char(c) => self.push_input(c),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn push_input(&mut self, pressed_buttons: char) {
        self.input.push(pressed_buttons);
    }

}

