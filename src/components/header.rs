use crate::action::Action;

use super::Component;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::style::Color;
use ratatui::layout::Rect;
use ratatui::style::Style;
use tui_textarea::Input;
use tui_textarea::Key;
use tui_textarea::TextArea;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;

pub struct Header<'a>  {
    pub key_value_textarea: Vec<TextArea<'a>>,
    pub selected_textarea: u8,
    pub selected: bool,
}

impl<'a> Header<'a> {
    pub fn new() -> Self {
        let mut key_textarea = TextArea::default();
        key_textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Key"));

        let mut value_textarea = TextArea::default();
        value_textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Value"));

        Self {
            key_value_textarea: vec![key_textarea, value_textarea],
            selected_textarea: 0,
            selected: false,
        }
    }

    pub fn new_with_key_value_pair(key_value_string: String) -> Self {
        if key_value_string.len() == 1 && key_value_string.contains(":") {
            return Header::new();
        }

        // should consider refactoring to save key and value as seperate
        // json objects instead as : could be contained in header?
        let key_value: Vec<&str> = key_value_string.split(":").collect();

        let mut key_textarea = TextArea::default();
        key_textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Key"));

        // can this be 0?
        key_textarea.insert_str(key_value[0]);

        let mut value_textarea = TextArea::default();
        value_textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Value"));

        // can this be 1?
        value_textarea.insert_str(key_value[1]);

        Self {
            key_value_textarea: vec![key_textarea, value_textarea],
            selected_textarea: 0,
            selected: false,
        }
    }

    pub fn toggle_selected_textarea(&mut self) -> Option<Action> {
        self.key_value_textarea[self.selected_textarea as usize].set_style(Style::default());
        self.selected_textarea = (self.selected_textarea + 1) % 2;
        self.key_value_textarea[self.selected_textarea as usize].set_style(Style::default().fg(Color::Red));
        None
    }

    pub fn get_key_value(&mut self) -> String {
        if self.key_value_textarea[0 as usize].lines().len() < 1 {
            String::new()
        } else {
            let key_as_string = self.key_value_textarea[0 as usize].lines()[0].as_str();
            let value_as_string = self.key_value_textarea[1 as usize].lines()[0].as_str();
            [key_as_string, value_as_string].join(":")
        }
    }

    pub fn handle_selected(&mut self) {
        self.selected = true;
    }
}

impl<'a> Component for Header<'a> {
    fn handle_key_events(&mut self) -> Option<Action> {
        let event_result = crossterm::event::read();
        match event_result {
            Ok(event) => {
                match event.into() {
                    Input { key: Key::Esc, .. } => self.handle_deselect(),
                    Input { key: Key::Tab, .. } => self.toggle_selected_textarea(),
                    input => {
                        self.key_value_textarea[self.selected_textarea as usize].input(input);
                        None
                    }
                }
            }
            Err(_) => Some(Action::Suspend)
        }
    }

    fn handle_deselect(&mut self) -> Option<Action> {
        self.key_value_textarea[self.selected_textarea as usize].set_style(Style::default());
        self.selected = false;
        Some(Action::Suspend)
    }

    fn handle_select(&mut self) {
        self.selected = true;
    }

    fn render_frame(&mut self, frame: &mut ratatui::prelude::Frame<'_>, area: Rect) -> std::io::Result<()> {

        let row_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Percentage(100),
            ]

        ).split(area);

        let mut row_block = Block::default()
            .title("");

        if self.selected {
            row_block = row_block.style(Style::default().bg(Color::Cyan));
        }


        let main_layout = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ],
        ).split(row_layout[0]);

        frame.render_widget(row_block, row_layout[0]);

        frame.render_widget(self.key_value_textarea[0 as usize].widget(), main_layout[0]);
        frame.render_widget(self.key_value_textarea[1 as usize].widget(), main_layout[1]);
        Ok(())
    }
}
