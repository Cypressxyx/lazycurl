use std::usize;

use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::Line, widgets::{Block, Borders, Tabs, Widget}};
use strum::{EnumIter, IntoEnumIterator, Display};

use crate::action::Action;

use super::{headers::Headers, Component};


#[derive(EnumIter, Display, Copy, Clone)]
pub enum SelectedTab {
    Headers,
    Body,
}

impl SelectedTab {

    fn title(self) -> Line<'static> {
        format!(" {self} ")
            .into()
    }
}

pub struct Parameters<'a> {
    pub selected: bool,
    pub headers_component: Headers<'a>,
    pub selected_tab: SelectedTab,
}

impl<'a> Parameters<'a> {
    pub fn new() -> Self {
        Self {
            selected: false,
            headers_component: Headers::new(),
            selected_tab: SelectedTab::Headers,
        }
    }

    pub fn new_with_headers(headers: Vec<String>) -> Self {
        Self {
            selected: false,
            headers_component: Headers::new_with_headers(headers),
            selected_tab: SelectedTab::Headers,
        }
    }

    pub fn get_headers(&mut self) -> Vec<String> {
        self.headers_component.get_key_values()
    }
}

impl<'a> Component for Parameters<'a> {
    fn handle_key_events(&mut self) -> Option<Action> {
        let event = match self.selected_tab {
            SelectedTab::Headers => self.headers_component.handle_key_events(),
            _ => None,
        };
        match event {
            Some(Action::Suspend) => self.handle_deselect(),
            _ => None,
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
            .margin(1)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(90),
            ])
            .split(area);

        let tab_titles = SelectedTab::iter().map(SelectedTab::title);

        let mut border_style = Style::default();

        if self.selected {
            border_style.fg = Some(Color::Green);
        }

        Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL).title("Parameters").border_style(border_style))
            .highlight_style(Color::Yellow)
            .select(self.selected_tab as usize)
            .divider("|")
            .render(area, frame.buffer_mut());

        let _ = match self.selected_tab {
            SelectedTab::Headers => self.headers_component.render_frame(frame, layout[1]),
            _ => Ok(()),
        };

        Ok(())

    }
}
