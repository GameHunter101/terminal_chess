use std::time::Duration;

use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};

mod render;
mod terminal_management;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let terminal = terminal_management::Terminal::default();
    let mut renderer = render::Render::new()?;
    renderer.add_text(
        render::TextFormats::MultiLine(vec!["first line","second line", "third line"]),
        render::InsertHorizontalPosition::Center,
        render::InsertVerticalPosition::Center,
        2,
    )?;
    renderer.render()?;

    terminal::enable_raw_mode().expect("Could not turn on raw mode");
    while terminal.run()? {}
    Ok(())
}
