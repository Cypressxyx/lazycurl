use std::usize;

use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::Line, widgets::{Block, Borders, Tabs, Widget}};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

use crate::{action::Action, utils::tui_block::main_block};

use super::{body::Body, headers::Headers, Component};


#[derive(EnumIter, Display, Copy, Clone, FromRepr)]
pub enum SelectedTab {
    Headers,
    Body,
}

impl SelectedTab {

    fn title(self) -> Line<'static> {
        format!(" {self} ")
            .into()
    }

    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    fn previous(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_sub(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

pub struct Parameters<'a> {
    pub selected: bool,
    pub headers_component: Headers<'a>,
    pub body_component: Body<'a>,
    pub selected_tab: SelectedTab,
}

impl<'a> Parameters<'a> {
    pub fn new() -> Self {
        Self {
            selected: false,
            headers_component: Headers::new(),
            selected_tab: SelectedTab::Headers,
            body_component: Body::new(),
        }
    }

    pub fn new_with_headers(headers: Vec<String>) -> Self {
        Self {
            selected: false,
            headers_component: Headers::new_with_headers(headers),
            selected_tab: SelectedTab::Headers,
            body_component: Body::new(),
        }
    }

    pub fn get_headers(&mut self) -> Vec<String> {
        self.headers_component.get_key_values()
    }

    pub fn get_body(&mut self) -> &str {
        self.body_component.get_body_text()
    }

    pub fn previous_tab(&mut self) -> Option<Action> {
        self.selected_tab = self.selected_tab.previous();
        None
    }

    pub fn next_tab(&mut self) -> Option<Action> {
        self.selected_tab = self.selected_tab.next();
        None
    }
}

impl<'a> Component for Parameters<'a> {
    fn handle_key_events(&mut self) -> Option<Action> {
        let event = match self.selected_tab {
            SelectedTab::Headers => self.headers_component.handle_key_events(),
            SelectedTab::Body => self.body_component.handle_key_events(),
        };
        match event {
            Some(Action::Suspend) => self.handle_deselect(),
            Some(Action::TabRight) => self.next_tab(),
            Some(Action::TabLeft) => self.previous_tab(),
            some_action => {
                if some_action.is_some() {
                    self.handle_deselect();
                    return some_action;
                }
                None
            },
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
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(90),
            ])
            .split(area);

        let tab_titles = SelectedTab::iter().map(SelectedTab::title);

        let block = main_block(&self.selected, "[2]-Parameters");

        Tabs::new(tab_titles)
            .block(block)
            .highlight_style(Color::Yellow)
            .select(self.selected_tab as usize)
            .divider("|")
            .render(area, frame.buffer_mut());

        let _ = match self.selected_tab {
            SelectedTab::Headers => self.headers_component.render_frame(frame, layout[1]),
            SelectedTab::Body => self.body_component.render_frame(frame, layout[1])
        };

        Ok(())

    }
}
