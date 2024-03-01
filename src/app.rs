use std::io::{self, Read};
use ratatui::{layout::{Constraint, Direction, Layout}, Frame};
use tui_textarea::{Input, Key};
use crate::{action::Action, components::{headers::Headers, response::Response, submit::Submit, url::Url, Component}, tui};
use curl::easy::Easy;

pub enum SelectedComponent {
    Main,
    Url,
    Submit,
    Response,
    Headers
}

pub struct App<'a> {
    pub exit: bool,
    pub url_component: Url<'a>,
    pub submit_component: Submit,
    pub selected_component: SelectedComponent,
    pub response_component: Response,
    pub header_component: Headers<'a>,
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
                    _ => ()
                }
            };
        }
        Ok(())
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
    }

    fn handle_curl_request(&mut self) {
        self.reset_selected_component();

        let mut headers = curl::easy::List::new();
        headers.append(self.header_component.get_key_value().as_str()).unwrap();
        curl(self.url_component.get_url(), &mut self.response, headers);
        self.response_component.update_response_value(String::from_utf8(self.response.clone()).unwrap())
    }
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

