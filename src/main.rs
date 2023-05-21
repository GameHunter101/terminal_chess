#![allow(unused)]

use std::io;

use crossterm::terminal;
use crossterm::event::{self,Event,KeyCode,KeyEvent};

mod contents;
mod render;
mod terminal_management;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = terminal_management::Terminal::new()?;

    // TODO:
    // Extract screens and text over here
    // Make renderer::new() take a screen as a parameter, or new_function renderer::new_template() make a blank screen with some text
    // New function renderer::new_screen(screen:Screen) will append a new screen
    // New function renderer::set_screen(screen_index: usize) will set the current screen to screen_index
    // Find a way to set foreground and background color for each text

    let mut renderer = render::Render::new()?;

    terminal::enable_raw_mode().expect("Could not turn on raw mode");
    while terminal.run().unwrap() {}
    Ok(())
}