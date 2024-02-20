use std::io::{self, stdout};
use tui_textarea::{TextArea, Input, Key};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

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
    let mut should_quit = false;
    while !should_quit {
        terminal.draw(|frame| {

            let main_layout = Layout::new(
                Direction::Horizontal,
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ],
            ).split(frame.size());

    
            let left_frame = Layout::new(
                Direction::Vertical,
                [
                    Constraint::Percentage(3),
                    Constraint::Percentage(97)
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
                Paragraph::new("")
                    .block(Block::default().title("Response").borders(Borders::ALL)),
                left_frame[1],
            );
        })?;
        should_quit = handle_events()?;

        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            input => {
                textarea.input(input);
            }
        }
    }

    disable_raw_mode()?;

    stdout().execute(LeaveAlternateScreen);
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let  Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true)
            }
        }
    }

    Ok(false)
}
