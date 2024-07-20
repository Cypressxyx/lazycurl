use std::usize;

use ratatui::{layout::Rect, style::{Color, Style}, symbols::scrollbar, text::Line, widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState}};
use tui_textarea::{Input, Key};

use crate::{action::Action, http_method::HTTPMethod, lazycurl_file:: LazyCurlFile, utils::tui_block::main_block};

use super::Component;

pub struct History {
    selected_file: Option<LazyCurlFile>,
    currently_selected_file: usize,
    lazycurl_files: Vec<LazyCurlFile>,
    scrollbar_state: ScrollbarState,
    selected: bool,
}

impl History {
    pub fn new() -> Self {
        Self {
            selected_file: None,
            currently_selected_file: 0,
            lazycurl_files: Vec::new(),
            scrollbar_state: ScrollbarState::new(0),
            selected: false
        }
    }


    pub fn take_selected_file(&mut self) -> Option<LazyCurlFile> {
        self.selected_file.take()
    }

    fn handle_load_request(&mut self) -> Option<Action> {
        if self.lazycurl_files.len() < 1 {
            self.handle_deselect()
        } else {
            self.selected_file = Some(self.lazycurl_files.get(self.currently_selected_file).unwrap().clone());
            Some(Action::LazycurlFileLoadRequest)
        }
    }

    fn handle_traverse_up_request(&mut self) -> Option<Action> {
        if self.currently_selected_file > 0  {
            self.currently_selected_file -= 1;
        }

        self.scrollbar_state = self.scrollbar_state.position(self.currently_selected_file);
        None
    }

    fn handle_traverse_down_request(&mut self) -> Option<Action>{
        if self.lazycurl_files.len() == 0 {
            return None;
        }

        if self.currently_selected_file < (self.lazycurl_files.len() - 1)  {
            self.currently_selected_file += 1;
        }

        self.scrollbar_state = self.scrollbar_state.position(self.currently_selected_file);

        None
    }

    pub fn get_lazycurl_files(&mut self) {
        // this is used to create history dir but is bad, refactor out to own
        // create histor method
        self.lazycurl_files = LazyCurlFile::new(
            String::new(), Vec::<String>::new(), HTTPMethod::POST
        ).get_history_lazycurlfiles().unwrap();

        self.scrollbar_state = self.scrollbar_state.content_length(self.lazycurl_files.len())
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
        self.selected = false;
        self.currently_selected_file = 0;
        Some(Action::Suspend)
    }

    fn handle_select(&mut self) {
        self.selected = true;
        self.currently_selected_file = 0;
    }

    fn render_frame(&mut self, frame: &mut ratatui::prelude::Frame<'_>, area: Rect) -> std::io::Result<()> {
        self.get_lazycurl_files();

        if self.lazycurl_files.is_empty() {
            let mut p = Paragraph::new("No history found")
                    .block(Block::default().title("History").borders(Borders::ALL));
            p = p.style(Style::default().bg(Color::Red));
            frame.render_widget(p, area);
            return Ok(())
        }

        let block = main_block(&self.selected, "History");

        let paragraph = Paragraph::new(self.lazycurl_files
                .iter()
                .enumerate()
                .map(|(index, f)| {
                        if index == self.currently_selected_file {
                            Line::from(f.url.clone()).style(Style::default().bg(Color::Blue))
                        } else {
                            Line::from(f.url.clone())
                        }
                })
                .collect::<Vec<_>>())
            .block(block)
            .scroll((self.currently_selected_file as u16, 0));

        frame.render_widget(paragraph, area);

        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .symbols(scrollbar::VERTICAL)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            area,
            &mut self.scrollbar_state);

        Ok(())
    }
}
