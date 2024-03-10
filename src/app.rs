use std::{io::{self}, str::FromStr};
use ratatui::{layout::{Constraint, Direction, Layout}, Frame};
use tui_textarea::{Input, Key};
use crate::{
    action::Action, components::{
        history::History,
        http_method::HTTPMethod,
        parameters::Parameters,
        response::Response,
        submit::Submit,
        url::Url, Component
    }, lazycurl_file::LazyCurlFile, tui, utils::{curl_service::curl_call, tui_frame_util::centered_rect}
};

#[derive(PartialEq)]
pub enum SelectedComponent {
    Main,
    Url,
    Submit,
    Response,
    History,
    Parameters,
}

pub struct App<'a> {
    pub exit: bool,
    pub url_component: Url<'a>,
    pub submit_component: Submit,
    pub response_component: Response,
    pub history_component: History,
    pub parameters_component: Parameters<'a>,
    pub httpmethod_component: HTTPMethod,
    pub selected_component: SelectedComponent,
    pub response: Vec<u8>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        Self {
            exit: false,
            url_component: Url::new(),
            submit_component: Submit::new(),
            response_component: Response::new(),
            history_component: History::new(),
            selected_component: SelectedComponent::Main,
            parameters_component: Parameters::new(),
            httpmethod_component: HTTPMethod::new(),
            response: Vec::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            if let Some(action) = self.handle_key_events() {
                self.handle_action(action);
            };
        }
        Ok(())
    }

    pub fn handle_lazcurlfile_load_request(&mut self) {
        if let Some(selected_file) = self.history_component.take_selected_file() {
            self.url_component = Url::new_withurl(selected_file.url);
            self.parameters_component = Parameters::new_with_headers(selected_file.headers);
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
            SelectedComponent::Response => self.response_component.handle_key_events(),
            SelectedComponent::History => self.history_component.handle_key_events(),
            SelectedComponent::Parameters => self.parameters_component.handle_key_events(),
        }
    }

    fn handle_exit(&mut self) {
        self.exit = true;
    }

    fn reset_selected_component(&mut self) {
        self.selected_component = SelectedComponent::Main;
    }

    fn handle_action(&mut self, action: Action) {
        match action {
            Action::Suspend => self.reset_selected_component(),
            Action::CurlRequest => self.handle_curl_request(),
            Action::LazycurlFileLoadRequest => self.handle_lazcurlfile_load_request(),
            Action::TabLeft => (),
            Action::TabRight => (),
            Action::Window1Request => {
                self.url_component.handle_select();
                self.selected_component = SelectedComponent::Url;
            }
            Action::Window2Request => {
                self.submit_component.handle_select();
                self.selected_component = SelectedComponent::Submit

            }
            Action::Window3Request => {
                self.parameters_component.handle_select();
                self.selected_component = SelectedComponent::Parameters
            }
            Action::Window4Request => {
                self.response_component.handle_select();
                self.selected_component = SelectedComponent::Response
            }
            Action::HistoryRequest => {
                self.history_component.handle_select();
                self.selected_component = SelectedComponent::History;
            },
        }
    }

    fn handle_component_selection(&mut self) -> io::Result<()> {
        match crossterm::event::read()?.into() {
            Input { key: Key::Char('q'), .. } => self.handle_exit(),
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
                self.parameters_component.handle_select();
                self.selected_component = SelectedComponent::Parameters
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
                Constraint::Percentage(10),
                Constraint::Percentage(30),
                Constraint::Percentage(60),
            ],
        ).split(frame.size());

        let url_frame = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(5),
                Constraint::Percentage(90),
                Constraint::Percentage(5),
            ],
        ).split(main_layout[0]);

        let _ = self.httpmethod_component.render_frame(frame, url_frame[0]);
        let _  = self.submit_component.render_frame(frame, url_frame[2]);
        let _  = self.parameters_component.render_frame(frame, main_layout[1]);
        let _  = self.response_component.render_frame(frame, main_layout[2]);
        let _  = self.url_component.render_frame(frame, url_frame[1]);

        if self.selected_component == SelectedComponent::History {
            let area = centered_rect(60, 25, frame.size());
            let _ = self.history_component.render_frame(frame, area);
        }
    }

    fn handle_curl_request(&mut self) {
        self.reset_selected_component();

        let mut headers = curl::easy::List::new();
        let component_headers = self.parameters_component.get_headers();
        component_headers
            .iter()
            .for_each(|f| headers.append(f).unwrap());
        self.response = Vec::new();
        let url = self.url_component.get_url().to_owned();
        let method = self.url_component.get_method();

        curl_call(url.as_str(), &mut self.response, headers, self.parameters_component.get_body(), method);
        let response_string = String::from_utf8(self.response.clone()).unwrap();
        self.response_component.update_response_value(response_string.clone());
        save_request(url.as_str(), component_headers)
    }

}

fn save_request(url: &str, headers: Vec<String>) {
    let _ = LazyCurlFile::new(String::from_str(url).unwrap(), headers).save();
}

