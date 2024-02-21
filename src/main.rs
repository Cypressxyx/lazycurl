use std::io::{self, stdout};
use tui_textarea::{TextArea, Input, Key};
use crossterm::{
    event::KeyCode, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand
};
use ratatui::{prelude::*, widgets::*};
use curl::easy::{self, Easy};

enum SelectedWindow {
    EXPLORER,
    URI,
    SUBMIT,
    HEADERS,
    HEADER_KEY,
    RESPONSE
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)  ?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut textarea = TextArea::default();
    let mut textarea_header_key = TextArea::default();
    let mut textarea_header_cookies = TextArea::default();
    
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("URI"),
    );

    textarea_header_key.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("key"),
    );

    textarea_header_cookies.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("value"),
    );
    let mut selected_window: SelectedWindow = SelectedWindow::EXPLORER;
    let mut data = Vec::new();

    loop {
        terminal.draw(|frame| {

            let main_layout = Layout::new(
                Direction::Horizontal,
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80),
                ],
            ).split(frame.size());

    
            let left_frame = Layout::new(
                Direction::Vertical,
                [
                    Constraint::Percentage(5),
                    Constraint::Percentage(10),
                    Constraint::Percentage(85),
                ],
            ).split(main_layout[1]);

            frame.render_widget(
                Paragraph::new("explorer stuff")
                    .block(Block::default().title("Explorer").borders(Borders::ALL)),
                main_layout[0],
            );

            let uri_frame = Layout::new(
                Direction::Horizontal,
                [
                    Constraint::Percentage(5),
                    Constraint::Percentage(90),
                    Constraint::Percentage(5),
                ],
            ).split(left_frame[0]);
            frame.render_widget(
                Paragraph::new("POST")
                    .block(Block::default().title("http method").borders(Borders::ALL)),
                uri_frame[0],
            );
            frame.render_widget(textarea.widget(), uri_frame[1]);
            frame.render_widget(
                Paragraph::new("Send")
                    .block(Block::default().title("").borders(Borders::ALL)),
                uri_frame[2],
            );

            /********** params ******/
            let header_params_frame = Layout::new(
                Direction::Horizontal,
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]
            ).split(left_frame[1]);
            frame.render_widget(textarea_header_key.widget(), header_params_frame[0]);
            frame.render_widget(textarea_header_cookies.widget(), header_params_frame[1]);

            /********** end params ******/



            frame.render_widget(
                Paragraph::new(String::from_utf8(data.clone()).unwrap())
                    .block(Block::default().title("Response").borders(Borders::ALL)),
                left_frame[2],
            );
        })?;
        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            Input { key: Key::Char('1'), .. } => {
                selected_window = SelectedWindow::EXPLORER;
            },
            Input { key: Key::Char('3'), .. } => {
                selected_window = SelectedWindow::URI;
            },
            Input { key: Key::Char('4'), .. } => {
                selected_window = SelectedWindow::SUBMIT;
            },
            Input { key: Key::Char('5'), .. } => {
                selected_window = SelectedWindow::HEADER_KEY;
            },
            Input { key: Key::Char('6'), .. } => {
                selected_window = SelectedWindow::HEADERS;
            },
            input => {
                match selected_window {
                    SelectedWindow::URI => textarea.input(input),
                    SelectedWindow::HEADER_KEY => textarea_header_key.input(input),
                    SelectedWindow::HEADERS => textarea_header_cookies.input(input),
                    SelectedWindow::SUBMIT => {
                        match input {
                            Input { key: Key::Enter, .. } => {
                                let mut headers = curl::easy::List::new();
                                let header_value_as_string = textarea_header_cookies.lines()[0].as_str();
                                let header_key = textarea_header_key.lines()[0].as_str();
                                headers.append([header_key, header_value_as_string].join(":").as_str()).unwrap();
                                curl(textarea.lines()[0].as_str(), &mut data, headers);
                                true
                            },
                            _ => true,
                        }
                    }
                    _ => true,
                };
            }
        };
    }

    disable_raw_mode()?;

    stdout().execute(LeaveAlternateScreen);
    Ok(())
}

fn curl(url: &str, data: &mut Vec<u8>, headers: curl::easy::List) {
    let mut post_data = "".as_bytes();
    let mut easy = Easy::new();
    easy.post(true).unwrap();
    easy.url(url).unwrap();
    easy.http_headers(headers).unwrap();
    easy.post_field_size(post_data.len() as u64).unwrap();
    let mut transfer = easy.transfer();
    transfer.write_function(|d| {
        data.extend_from_slice(d);
        Ok(d.len())
    }).unwrap();

    transfer.perform().unwrap();
}
