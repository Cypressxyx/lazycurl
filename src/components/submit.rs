use tui_textarea::{Input, Key};
use ratatui::{layout::Rect, style::{Color, Style}, widgets::{Block, Borders, Paragraph}};

use crate::action::Action;

use super::Component;

pub struct Submit {
    pub selected: bool,
}

impl Submit {
    pub fn new() -> Self {
        Self {
            selected: false,
        }
    }
}

impl Component for Submit {
    fn handle_key_events(&mut self) -> Option<Action> {
        let event_result = crossterm::event::read();
        match event_result {
            Ok(event) => {
                match event.into() {
                    Input { key: Key::Esc, .. } => self.handle_deselect(),
                    Input { key: Key::Enter, .. } => {
                        self.handle_deselect();
                        Some(Action::CurlRequest)
                    },
                    Input { key: Key::Char('h'), .. } => {
                        self.handle_deselect();
                        Some(Action::HistoryRequest)
                    },
                    Input { key: Key::Char('1'), .. } => {
                        self.handle_deselect();
                        Some(Action::Window1Request)
                    },
                    Input { key: Key::Char('2'), .. } => {
                        self.handle_deselect();
                        Some(Action::Window2Request)
                    },
                    Input { key: Key::Char('3'), .. } => {
                        self.handle_deselect();
                        Some(Action::Window3Request)
                    },
                    Input { key: Key::Char('4'), .. } => {
                        self.handle_deselect();
                        Some(Action::Window4Request)
                    },

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
        if self.selected {
            let p = Paragraph::new("Send")
                        .block(Block::default().title("").borders(Borders::ALL).style(Style::default().fg(Color::Green)));
            frame.render_widget(p, area);

        } else {
            let p = Paragraph::new("Send")
                        .block(Block::default().title("").borders(Borders::ALL));
            frame.render_widget(p, area);
        }
        Ok(())
    }
}
