use ratatui::{layout::Rect, style::{Color, Style, Styled, Stylize}, widgets::{Block, Borders, Paragraph}};
use serde_json::Value;
use tui_textarea::{Input, Key};

use crate::{action::Action, lazycurl_file::{self, LazyCurlFile}};

use super::Component;

pub struct History {
}

impl History {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn get_lazycurl_files(&mut self) -> Vec<LazyCurlFile> {
        LazyCurlFile::new(String::new()).get_history_lazycurlfiles().unwrap()
    }
}

impl Component for History {
    fn handle_key_events(&mut self) -> Option<Action> {
        let event_result = crossterm::event::read();
        match event_result {
            Ok(event) => {
                match event.into() {
                    Input { key: Key::Esc, .. } => self.handle_deselect(),
                    _ => None
                }
            }
            Err(_) => Some(Action::Suspend)
        }
    }


    fn handle_deselect(&mut self) -> Option<Action> {
        Some(Action::Suspend)
    }

    fn handle_select(&mut self) {
    }

    fn render_frame(&mut self, frame: &mut ratatui::prelude::Frame<'_>, area: Rect) -> std::io::Result<()> {
        let lazycurl_files = self.get_lazycurl_files();
        let urls: Vec<String> = lazycurl_files.into_iter()
            .map(|f| f.url)
            .collect();

        let mut p = Paragraph::new(urls.join(","))
                    .block(Block::default().title("History").borders(Borders::ALL));
        p = p.style(Style::default().bg(Color::Red));
        frame.render_widget(p, area);
        Ok(())
    }
}
