use std::{io::{self, Read}, str::FromStr};
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, Frame};
use tui_textarea::{Input, Key};
use crate::{action::Action, components::{headers::Headers, history::History, response::Response, submit::Submit, url::Url, Component}, lazycurl_file::LazyCurlFile, tui};
use curl::easy::Easy;

#[derive(PartialEq)]
pub enum SelectedComponent {
    Main,
    Url,
    Submit,
    Response,
    Headers,
    History,
}

pub struct App<'a> {
    pub exit: bool,
    pub url_component: Url<'a>,
    pub submit_component: Submit,
    pub selected_component: SelectedComponent,
    pub response_component: Response,
    pub header_component: Headers<'a>,
    pub history_component: History,
    pub response: Vec<u8>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        Self {
            exit: false,
            url_component: Url::new(),
            submit_component: Submit::new(),
            response_component: Response::new(),
            header_component: Headers::new(),
            history_component: History::new(),
            selected_component: SelectedComponent::Main,
            response: Vec::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            if let Some(action) = self.handle_key_events() {
                match action {
                    Action::Suspend => self.reset_selected_component(),
                    Action::CurlRequest => self.handle_curl_request(),
                    Action::LazycurlFileLoadRequest => self.handle_lazcurlfile_load_request(),
                }
            };
        }
        Ok(())
    }

    pub fn handle_lazcurlfile_load_request(&mut self) {
        if let Some(selected_file) = self.history_component.take_selected_file() {
            self.url_component = Url::new_withurl(selected_file.url);
        }
        self.reset_selected_component()
    }

    pub fn handle_key_events(&mut self) -> Option<Action> {
        match self.selected_component {
            SelectedComponent::Main => {
                let _ = self.handle_component_selection();
                None
            }
            SelectedComponent::Url => self.url_component.handle_key_events(),
            SelectedComponent::Submit => self.submit_component.handle_key_events(),
            SelectedComponent::Headers => self.header_component.handle_key_events(),
            SelectedComponent::Response => self.response_component.handle_key_events(),
            SelectedComponent::History => self.history_component.handle_key_events(),
        }
    }

    fn handle_exit(&mut self) {
        self.exit = true;
    }

    fn reset_selected_component(&mut self) {
        self.selected_component = SelectedComponent::Main;
    }

    fn handle_component_selection(&mut self) -> io::Result<()> {
        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => self.handle_exit(),
            Input { key: Key::Char('h'), .. } => {
                self.history_component.handle_select();
                self.selected_component = SelectedComponent::History;
            },
            Input { key: Key::Char('1'), .. } => {
                self.url_component.handle_select();
                self.selected_component = SelectedComponent::Url;
            },
            Input { key: Key::Char('2'), .. } => {
                self.submit_component.handle_select();
                self.selected_component = SelectedComponent::Submit
            },
            Input { key: Key::Char('3'), .. } => {
                self.header_component.handle_select();
                self.selected_component = SelectedComponent::Headers
            },
            Input { key: Key::Char('4'), .. } => {
                self.response_component.handle_select();
                self.selected_component = SelectedComponent::Response
            }
            _ => ()
        }

        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let main_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Percentage(8),
                Constraint::Percentage(30),
                Constraint::Percentage(62),
            ],
        ).split(frame.size());

        let url_frame = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(90),
                Constraint::Percentage(10),
            ],
        ).split(main_layout[0]);

        let _  = self.url_component.render_frame(frame, url_frame[0]);
        let _  = self.submit_component.render_frame(frame, url_frame[1]);
        let _  = self.header_component.render_frame(frame, main_layout[1]);
        let _  = self.response_component.render_frame(frame, main_layout[2]);

        if self.selected_component == SelectedComponent::History {
            let area = centered_rect(60, 25, frame.size());
            let _ = self.history_component.render_frame(frame, area);
        }
    }

    fn handle_curl_request(&mut self) {
        self.reset_selected_component();

        let mut headers = curl::easy::List::new();
        headers.append(self.header_component.get_key_value().as_str()).unwrap();
        self.response = Vec::new();
        let url = self.url_component.get_url();
        curl(url, &mut self.response, headers);
        let response_string = String::from_utf8(self.response.clone()).unwrap();
        self.response_component.update_response_value(response_string.clone());
        save_request(url, response_string.clone())
    }

}

fn save_request(url: &str, response: String) {
    let _ = LazyCurlFile::new(String::from_str(url).unwrap()).save();
}

fn curl(url: &str, data: &mut Vec<u8>, headers: curl::easy::List) {
        let mut post_data = "{}".as_bytes();
        let mut easy = Easy::new();
        easy.post(true).unwrap();
        easy.url(url).unwrap();
        easy.http_headers(headers).unwrap();
        easy.post_field_size(post_data.len() as u64).unwrap();
        let mut transfer = easy.transfer();

        transfer.read_function(|buf| {
            Ok(post_data.read(buf).unwrap_or(0))
        }).unwrap();


        transfer.write_function(|d| {
            data.extend_from_slice(d);
            Ok(d.len())
        }).unwrap();

        transfer.perform().unwrap();
     }


/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
