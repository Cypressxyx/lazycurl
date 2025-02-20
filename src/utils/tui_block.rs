use ratatui::widgets::{ Borders, Block };
use ratatui::prelude::{ Style, Modifier };
use ratatui::style::Color;

pub fn main_block<'a>(selected: &bool, title: &'a str) -> Block<'a> {
    // Highlight color
    let border_style = if *selected {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };

    // Higlight title
    let title_style = if *selected {
        Style::default().add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_style(title_style)
        .border_style(border_style)
}
