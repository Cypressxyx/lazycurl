use std::io;

pub mod app;
pub mod tui;
pub mod components;
pub mod action;
pub mod lazycurl_file;
pub mod utils;

use crate::app::App;

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = App::new().run(&mut terminal);
    tui::restore()?;
    app_result
}

