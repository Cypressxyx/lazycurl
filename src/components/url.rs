use crate::action::Action;
use crate::utils::tui_frame_util::centered_rect;

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
    pub edit_mode: bool,
}

impl<'a> Url<'a> {
    pub fn new() -> Self {
        let mut text_area = TextArea::default();
        text_area.set_placeholder_text("Enter URL or paste text");
        text_area.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("URI (press e to edit)"));

        Self {
            url_text_area: text_area,
            edit_mode: false,
        }
    }

    pub fn new_withurl(url: String) -> Self {
        let mut text_area = TextArea::default();
        text_area.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("URI (press e to edit)"));

        text_area.insert_str(url);

        Self {
            url_text_area: text_area,
            edit_mode: false,
        }
    }
    pub fn get_url(&mut self) -> &str{
        self.url_text_area.lines()[0].as_str()
    }

    pub fn handle_edit_mode(&mut self) -> Option<Action> {
        self.edit_mode = true;
        None
    }

    pub fn handle_exit_edit_mode(&mut self) -> Option<Action> {
        self.edit_mode = false;
        None
    }

    pub fn handle_edit_mode_key_events(&mut self) -> Option<Action> {
        let event_result = crossterm::event::read();
        match event_result {
            Ok(event) => {
                match event.into() {
                    Input { key: Key::Esc, .. } => self.handle_exit_edit_mode(),
                    input => {
                        self.url_text_area.input(input);
                        None
                    }
                }
            }
            Err(_) => Some(Action::Suspend)
        }
    }

    pub fn render_edit_mode_frame(&mut self, frame: &mut ratatui::prelude::Frame<'_>) {
        let area = centered_rect(60, 25, frame.size());
        frame.render_widget(self.url_text_area.widget(), area);
    }
}

impl<'a> Component for Url<'a> {
    fn handle_key_events(&mut self) -> Option<Action> {
        if self.edit_mode {
            return self.handle_edit_mode_key_events()
        }

        let event_result = crossterm::event::read();
        match event_result {
            Ok(event) => {
                match event.into() {
                    Input { key: Key::Esc, .. } => self.handle_deselect(),
                    Input { key: Key::Char('e'), .. } => self.handle_edit_mode(),
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
        self.url_text_area.set_block(Block::default()
            .borders(Borders::ALL)
            .title("URI (press e to edit)")
            .border_style(Style::default()));
        Some(Action::Suspend)
    }

    fn handle_select(&mut self) {
        self.url_text_area.set_block(Block::default()
            .borders(Borders::ALL)
            .title("URI (press e to edit)")
            .border_style(Style::default().fg(Color::Green)));
    }

    fn render_frame(&mut self, frame: &mut ratatui::prelude::Frame<'_>, area: Rect) -> std::io::Result<()> {
        frame.render_widget(self.url_text_area.widget(), area);

        if self.edit_mode {
            self.render_edit_mode_frame(frame)
        }

        Ok(())
    }
}
