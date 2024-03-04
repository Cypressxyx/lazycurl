use std::usize;

use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, widgets::{Block, Borders, Paragraph}};
use tui_textarea::{Input, Key};

use crate::{action::Action, lazycurl_file:: LazyCurlFile};

use super::Component;

pub struct History {
    selected_file: Option<LazyCurlFile>,
    currently_selected_file: usize,
    lazycurl_files: Vec<LazyCurlFile>
}

impl History {
    pub fn new() -> Self {
        Self {
            selected_file: None,
            currently_selected_file: 0,
            lazycurl_files: Vec::new(),
        }
    }


    pub fn take_selected_file(&mut self) -> Option<LazyCurlFile> {
        self.selected_file.take()
    }

    fn handle_load_request(&mut self) -> Option<Action> {
        if self.lazycurl_files.len() < 1 {
            Some(Action::Suspend)
        } else {
            self.selected_file = Some(self.lazycurl_files.get(self.currently_selected_file).unwrap().clone());
            Some(Action::LazycurlFileLoadRequest)
        }
    }

    fn handle_traverse_up_request(&mut self) -> Option<Action> {
        if self.currently_selected_file != 0  {
            self.currently_selected_file -= 1;
        }

        None
    }

    fn handle_traverse_down_request(&mut self) -> Option<Action>{
        if self.currently_selected_file < self.lazycurl_files.len() - 1  {
            self.currently_selected_file += 1;
        }

        None
    }

    pub fn get_lazycurl_files(&mut self) {
        self.lazycurl_files = LazyCurlFile::new(String::new()).get_history_lazycurlfiles().unwrap()
    }
}

impl Component for History {
    fn handle_key_events(&mut self) -> Option<Action> {
        let event_result = crossterm::event::read();
        match event_result {
            Ok(event) => {
                match event.into() {
                    Input { key: Key::Esc, .. } => self.handle_deselect(),
                    Input { key: Key::Char('l'), .. } => self.handle_load_request(),
                    Input { key: Key::Enter, .. } => self.handle_load_request(),
                    Input { key: Key::Char(' '), .. } => self.handle_load_request(),
                    Input { key: Key::Char('j'), .. } => self.handle_traverse_down_request(),
                    Input { key: Key::Char('k'), .. } => self.handle_traverse_up_request(),
                    _ => None
                }
            }
            Err(_) => Some(Action::Suspend)
        }
    }

    fn handle_deselect(&mut self) -> Option<Action> {
        self.currently_selected_file = 0;
        Some(Action::Suspend)
    }

    fn handle_select(&mut self) {
        self.currently_selected_file = 0;
    }

    fn render_frame(&mut self, frame: &mut ratatui::prelude::Frame<'_>, area: Rect) -> std::io::Result<()> {
        self.get_lazycurl_files();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(self.lazycurl_files.iter().map(|_| Constraint::Length(3)).collect::<Vec<_>>())
            .split(area);

        for (i, lazy_curl_file) in self.lazycurl_files.iter().enumerate() {
            let mut p = Paragraph::new(lazy_curl_file.url.clone())
                    .block(Block::default().title("History").borders(Borders::ALL));
            p = p.style(Style::default().bg(Color::Red));
            if i == self.currently_selected_file {
                p = p.style(Style::default().bg(Color::Green));
            }

            frame.render_widget(p, layout[i]);
        }

        Ok(())
    }
}
