// src/main.rs
// Entry point + app loop

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    layout::{Constraint, Direction, Layout},
    prelude::Alignment,
    style::Stylize,
    style::{Color, Modifier, Style},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    widgets::{List, ListItem, ListState, Wrap},
};

mod app;
mod launcher;
use crate::{app::state::{App, InputMode}, launcher::appentry::AppEntry};

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
        // Split screen vertically into two chunks
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(90), // Top = list
                Constraint::Percentage(10), // Bottom = footer
            ])
            .split(frame.area());

        if self.filtered_apps.is_empty() {
            let empty = Paragraph::new("No matches found.");
            frame.render_widget(empty, chunks[0]);
            return;
        }

        // Build list of visible items
        let total_apps = self.all_apps.len();
        let visible_count = chunks[0].height.saturating_sub(1) as usize;

        let start = self
            .selected_index
            .saturating_sub(visible_count / 2)
            .min(total_apps.saturating_sub(visible_count));

        let end = (start + visible_count).min(total_apps);

        let list_items: Vec<ListItem> = self.filtered_apps[start..end]
            .iter()
            .map(|app| ListItem::new(app.name.clone()))
            .collect();

        let list = List::new(list_items)
            .highlight_symbol("→ ")
            .highlight_style(
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            );

        // Set list state with selected index relative to visible window
        let mut list_state = ListState::default();
        list_state.select(Some(self.selected_index.saturating_sub(start)));

        frame.render_stateful_widget(list, chunks[0], &mut list_state);

        // Draw footer bar
        // let footer = Paragraph::new("↑ ↓ to scroll • :q to quit")
        let footer_text = format!(
            "↑ ↓ to scroll • :q to quit \n Search: {} \n \n Command: {}",
            self.input, self.command_input
        );
        let footer = Paragraph::new(footer_text)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(footer, chunks[1]);
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
            InputMode::Insert => match key_event.code {
                KeyCode::Char(':') => {
                    self.mode = InputMode::Command;
                    self.command_input.clear();
                }

                KeyCode::Char(c) => {
                    self.input.push(c);
                    self.update_filter();
                }

                KeyCode::Backspace => {
                    self.input.pop();
                }

                KeyCode::Up => {
                    if self.selected_index > 0 {
                        self.selected_index -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.selected_index + 1 < self.all_apps.len() {
                        self.selected_index += 1;
                    }
                }

                _ => {}
            },
            InputMode::Command => match key_event.code {
                KeyCode::Char(c) => {
                    self.command_input.push(c);
                }

                KeyCode::Enter => {
                    if self.command_input == "q" || self.command_input == "quit" {
                        self.exit()
                    }
                }

                KeyCode::Esc => self.mode = InputMode::Insert,

                KeyCode::Backspace => {
                    self.command_input.pop();
                }

                _ => {}
            },
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }


    fn update_filter(&mut self) {
        let mut filtered = Vec::new();
        let matcher = SkimMatcherV2::default();

        for app in &self.all_apps {
            if let Some(_) = matcher.fuzzy_match(&app.name, &self.input) {
                filtered.push(app.clone());
            }
        }

        self.filtered_apps = filtered;
    }
}
