#![allow(unused)]

use crossterm::{terminal};

mod render;
mod terminal_management;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = terminal_management::Terminal::new()?;
    let mut renderer = render::Render::new()?;
    /* renderer.content.add_text(
        render::TextFormats::MultiLine(vec!["first line","second line", "third line"]),
        render::InsertHorizontalPosition::Center,
        render::InsertVerticalPosition::Center,
        2,
    )?; */

    terminal::enable_raw_mode().expect("Could not turn on raw mode");
    while terminal.run()? {}
    Ok(())
}
