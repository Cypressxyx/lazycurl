use crate::action::Action;
use crate::http_method::HTTPMethod;
use crate::utils::tui_block::main_block;
use crate::utils::tui_frame_util::centered_rect;

use super::Component;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::style::Color;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::Tabs;
use ratatui::widgets::Widget;
use strum::IntoEnumIterator;
use tui_textarea::Input;
use tui_textarea::Key;
use tui_textarea::TextArea;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;

pub struct Url<'a>  {
    pub url_text_area: TextArea<'a>,
    pub edit_mode: bool,
    pub http_method: HTTPMethod,
    pub selected: bool,
}

impl<'a> Url<'a> {
    pub fn new() -> Self {
        let mut text_area = TextArea::default();
        text_area.set_placeholder_text("Enter URL or paste text");
        text_area.set_block(
        Block::default()
            .title(""));

        Self {
            url_text_area: text_area,
            edit_mode: false,
            http_method: HTTPMethod::POST,
            selected: false,
        }
    }

    pub fn new_withurl_and_httpmethod(url: String, http_method: HTTPMethod) -> Self {
        let mut text_area = TextArea::default();
        text_area.set_block(
        Block::default()
            .title(""));

        text_area.insert_str(url);

        Self {
            url_text_area: text_area,
            edit_mode: false,
            selected: false,
            http_method,
        }
    }

    pub fn get_url(&mut self) -> &str{
        self.url_text_area.lines()[0].as_str()
    }

    pub fn get_method(&mut self) -> HTTPMethod {
        self.http_method
    }

    pub fn handle_edit_mode(&mut self) -> Option<Action> {
        self.url_text_area.set_block(Block::default()
            .borders(Borders::ALL)
            .title(" Editing URL ")
            .border_style(Style::default().fg(Color::Green)));
        self.edit_mode = true;
        None
    }

    pub fn handle_exit_edit_mode(&mut self) -> Option<Action> {
        self.url_text_area.set_block(Block::default()
            .title("")
            .border_style(Style::default()));

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

    pub fn handle_submit(&mut self) -> Option<Action> {
        self.handle_deselect();
        Some(Action::CurlRequest)
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
                    Input { key: Key::Enter, .. } => self.handle_submit(),                    Input { key: Key::Char('['), .. } => {
                        self.http_method = self.http_method.prev();
                        None
                    },
                    Input { key: Key::Char(']'), .. } => {
                        self.http_method = self.http_method.next();
                        None
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
        self.selected =  true;
    }

    fn render_frame(&mut self, frame: &mut ratatui::prelude::Frame<'_>, area: Rect) -> std::io::Result<()> {
        let http_method_lines = HTTPMethod::iter().map(HTTPMethod::line);
        let block = main_block(&self.selected, "[1]—URL—(press E to edit, enter to submit)");

        Tabs::new(http_method_lines)
            .block(block)
            .highlight_style(Color::LightYellow)
            .select(self.http_method as usize)
            .divider("|")
            .render(area, frame.buffer_mut());

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(100)])
            .split(area);

        frame.render_widget(self.url_text_area.widget(), layout[0]);
        if self.edit_mode {
            self.render_edit_mode_frame(frame)
        }
        Ok(())
    }
}
