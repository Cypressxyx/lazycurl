use crate::action::Action;

use super::Component;
use ratatui::style::Color;
use ratatui::layout::Rect;
use ratatui::style::Style;
use tui_textarea::Input;
use tui_textarea::Key;
use tui_textarea::TextArea;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;

pub struct Url<'a>  {
    pub url_text_area: TextArea<'a>,
}

impl<'a> Url<'a> {
    pub fn new() -> Self {
        let mut text_area = TextArea::default();
        text_area.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("URI"));

        Self {
            url_text_area: text_area,
        }
    }

    pub fn get_url(&mut self) -> &str{
        self.url_text_area.lines()[0].as_str()
    }
}

impl<'a> Component for Url<'a> {
    fn handle_key_events(&mut self) -> Option<Action> {
        let event_result = crossterm::event::read();
        match event_result {
            Ok(event) => {
                match event.into() {
                    Input { key: Key::Esc, .. } => self.handle_deselect(),
                    input => {
                        self.url_text_area.input(input);
                        None
                    }
                }
            }
            Err(_) => Some(Action::Suspend)
        }
    }

    fn handle_deselect(&mut self) -> Option<Action> {
        self.url_text_area.set_style(Style::default());
        Some(Action::Suspend)
    }

    fn handle_select(&mut self) {
        self.url_text_area.set_style(Style::default().fg(Color::Red));
    }

    fn render_frame(&mut self, frame: &mut ratatui::prelude::Frame<'_>, area: Rect) -> std::io::Result<()> {
        frame.render_widget(self.url_text_area.widget(), area);
        Ok(())
    }
}
