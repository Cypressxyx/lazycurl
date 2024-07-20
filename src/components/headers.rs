use crate::action::Action;

use super::header::Header;
use super::Component;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Paragraph;
use tui_textarea::Input;
use tui_textarea::Key;

pub struct Headers<'a>  {
    pub headers: Vec<Header<'a>>,
    pub selected_header_index: usize,
    pub is_in_edit_mode: bool,

}

impl<'a> Headers<'a> {
    pub fn new() -> Self {
        Self {
            headers: vec![Header::new()],
            selected_header_index:0,
            is_in_edit_mode: false,
        }
    }

    pub fn new_with_headers(headers: Vec<String>) -> Self {
        Self {
            headers: headers.into_iter().map(|x| Header::new_with_key_value_pair(x)).collect(),
            selected_header_index:0,
            is_in_edit_mode: false,
        }
    }

    pub fn get_key_values(&mut self) -> Vec<String> {
        self.headers.iter_mut()
            .map(|h| h.get_key_value())
            .collect::<Vec<_>>()
    }

    fn handle_traverse_up_request(&mut self) -> Option<Action> {
        if self.selected_header_index != 0  {
            self.selected_header_index -= 1;
        }

        None
    }

    fn handle_traverse_down_request(&mut self) -> Option<Action>{
        if self.selected_header_index < self.headers.len() - 1  {
            self.selected_header_index += 1;
        }

        None
    }

    fn handle_add_header(&mut self) -> Option<Action> {
        self.headers.push(Header::new());
        None
    }

    fn handle_edit_header(&mut self) ->Option<Action> {
        self.headers.get_mut(self.selected_header_index).unwrap().handle_selected();
        self.is_in_edit_mode = true;
        None
    }

    fn handle_delete_header(&mut self) -> Option<Action> {
        if self.headers.is_empty() {
            return None
        }
        self.headers.remove(self.selected_header_index);
        None
    }

    fn handle_headers_key_events(&mut self) -> Option<Action> {
        let event_result = crossterm::event::read();
        match event_result {
            Ok(event) => {
                match event.into() {
                    Input { key: Key::Esc, .. } => self.handle_deselect(),
                    Input { key: Key::Char('j'), .. } => self.handle_traverse_down_request(),
                    Input { key: Key::Char('k'), .. } => self.handle_traverse_up_request(),
                    Input { key: Key::Char('e'), .. } => self.handle_edit_header(),
                    Input { key: Key::Char('a'), .. } => self.handle_add_header(),
                    Input { key: Key::Char('d'), .. } => self.handle_delete_header(),
                    Input { key: Key::Char('['), .. } => Some(Action::TabLeft),
                    Input { key: Key::Char(']'), .. } => Some(Action::TabRight),
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
                    _ => None
                }
            }
            Err(_) => Some(Action::Suspend)
        }
    }
}

impl<'a> Component for Headers<'a> {
    fn handle_key_events(&mut self) -> Option<Action> {
        if self.is_in_edit_mode {
            let action = self.headers.get_mut(self.selected_header_index).unwrap().handle_key_events();

            if action.is_some_and(|a| a == Action::Suspend) {
                self.is_in_edit_mode = false;
            }
            None

        } else {
            self.handle_headers_key_events()
        }
    }

    fn handle_deselect(&mut self) -> Option<Action> {
        Some(Action::Suspend)
    }

    fn handle_select(&mut self) {
    }

    fn render_frame(&mut self, frame: &mut ratatui::prelude::Frame<'_>, area: Rect) -> std::io::Result<()> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(self.headers.iter().map(|_| Constraint::Length(3)).collect::<Vec<_>>())
            .split(area);

        for (i, header) in self.headers.iter_mut().enumerate() {
            let _ = header.render_frame(frame, layout[i]);
            if i == self.selected_header_index {
                frame.render_widget(Paragraph::new("").style(Style::default().bg(ratatui::style::Color::DarkGray)), layout[i]);
            }
        }

        Ok(())
    }
}
