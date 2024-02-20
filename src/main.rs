use std::io::{self, stdout};
use tui_textarea::{TextArea, Input, Key};
use crossterm::{
    event::KeyCode, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand
};
use ratatui::{prelude::*, widgets::*};
use curl::easy::Easy;

enum SelectedWindow {
    EXPLORER,
    URI,
    SUBMIT,
    RESPONSE
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)  ?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut textarea = TextArea::default();
    
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("URI"),
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
                    Constraint::Percentage(95)
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

            frame.render_widget(
                Paragraph::new(String::from_utf8(data.clone()).unwrap())
                    .block(Block::default().title("Response").borders(Borders::ALL)),
                left_frame[1],
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
            input => {
                match selected_window {
                    SelectedWindow::URI => textarea.input(input),
                    SelectedWindow::SUBMIT => {
                        match input {
                            Input { key: Key::Enter, .. } => {
                                curl(textarea.lines()[0].as_str(), &mut data);
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

fn curl(url: &str, data: &mut Vec<u8>) {
    println!("starting curl");
    let mut easy = Easy::new();
    easy.url(url).unwrap();


    let mut transfer = easy.transfer();
    transfer.write_function(|d| {
        data.extend_from_slice(d);
        Ok(d.len())
    }).unwrap();

    transfer.perform().unwrap();
}
