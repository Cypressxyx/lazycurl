use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, widgets::{Block, Borders, Paragraph}};
use serde_json::Value;
use tui_textarea::{Input, Key, TextArea};

use crate::action::Action;

use super::Component;

pub struct Body<'a> {
    pub selected: bool,
    pub body_textarea: TextArea<'a>,
    pub is_edit_mode: bool,

}

impl<'a> Body<'a> {
    pub fn new() -> Self {
        let mut text_area = TextArea::default();
        Block::default()
            .borders(Borders::ALL)
            .title("");

        Self {
            selected: false,
            body_textarea: text_area,
            is_edit_mode: false,
        }
    }

    pub fn get_body_text(&mut self) -> &str {
        self.body_textarea.lines()[0].as_str()
    }

    pub fn handle_edit_mode(&mut self) -> Option<Action> {
        self.is_edit_mode = true;
        None
    }

    pub fn handle_exit_edit_mode(&mut self) -> Option<Action> {
        self.is_edit_mode = false;
        None
    }

    pub fn handle_edit_mode_key_events(&mut self) -> Option<Action> {
        let event_result = crossterm::event::read();
        match event_result {
            Ok(event) => {
                match event.into() {
                    Input { key: Key::Esc, .. } => self.handle_exit_edit_mode(),
                    input => {
                        self.body_textarea.input(input);
                        None
                    }
                }
            }
            Err(_) => Some(Action::Suspend)
        }
    }
}

impl<'a> Component for Body<'a> {
    fn handle_key_events(&mut self) -> Option<Action> {
        if self.is_edit_mode {
            return self.handle_edit_mode_key_events()
        }

        let event_result = crossterm::event::read();
        match event_result {
            Ok(event) => {
                match event.into() {
                    Input { key: Key::Esc, .. } => self.handle_deselect(),
                    Input { key: Key::Char('['), .. } => Some(Action::TabLeft),
                    Input { key: Key::Char(']'), .. } => Some(Action::TabRight),
                    Input { key: Key::Char('e'), .. } => self.handle_edit_mode(),
                    _ => None
                }
            }
            Err(_) => Some(Action::Suspend)
        }
    }


    fn handle_deselect(&mut self) -> Option<Action> {
        self.selected = false;
        Some(Action::Suspend)
    }

    fn handle_select(&mut self) {
        self.selected = true;
    }

    fn render_frame(&mut self, frame: &mut ratatui::prelude::Frame<'_>, area: Rect) -> std::io::Result<()> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([Constraint::Percentage(100)])
            .split(area);
        frame.render_widget(self.body_textarea.widget(), layout[0]);
        Ok(())
    }
}
