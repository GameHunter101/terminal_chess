#![allow(unused)]

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal;
use render::Render;
use screen::{ButtonText, PlainText, Screen, Text};

mod render;
mod screen;
mod terminal_management;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut initial_screen = Screen::new();
    initial_screen
        .screen_rows
        .edit_single_row(Text::Plain(PlainText::new(
            "Text thing",
            initial_screen.width,
            initial_screen.height,
            screen::InsertHorizontalPosition::Center,
            screen::InsertVerticalPosition::Bottom,
        )));

    let mut renderer = render::Render::new(initial_screen)?;
    let mut new_screen = Screen::new()
        .screen_rows
        .edit_single_row(Text::Button(ButtonText::new(
            "Other Screen",
            initial_screen.width,
            initial_screen.height,
            screen::InsertHorizontalPosition::Right,
            screen::InsertVerticalPosition::Center,
            Box::new(|_: usize, _: usize| renderer.set_screen(1)),
        )));
    let mut terminal = terminal_management::Terminal::new(renderer);

    // TODO:
    // New function renderer::new_screen(screen:Screen) will append a new screen
    // New function renderer::set_screen(screen_index: usize) will set the current screen to screen_index
    // Find a way to set foreground and background color for each text

    terminal::enable_raw_mode().expect("Could not turn on raw mode");
    while terminal.run().unwrap() {}
    Ok(())
}

fn next_screen(render: &mut Render) {
    render.set_screen(1);
}

fn back_screen(render: &mut Render) {
    render.set_screen(1);
}
