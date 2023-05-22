#![allow(unused)]

use std::io;
use std::collections::HashMap;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal;
use render::Render;
use screen::{ButtonText, PlainText, Screen, Text};

mod render;
mod screen;
mod terminal_management;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = terminal::size().unwrap();
    let width = width as usize;
    let height = height as usize;
	let mut button_map: HashMap<&str, Box<dyn Fn()>> = HashMap::new();
    let mut initial_screen = Screen::new(button_map);
    initial_screen
        .screen_rows
        .edit_single_row(Text::Button(ButtonText::new(
            "Text thing",
            width,
            height,
            screen::InsertHorizontalPosition::Center,
            screen::InsertVerticalPosition::Bottom,
			"next_screen",
        )));

	(&mut initial_screen).button_map.insert("next_screen", Box::new(|| {crossterm::queue!(io::stdout(),crossterm::style::SetBackgroundColor(crossterm::style::Color::Rgb { r: 50, g: 0, b: 50 }));}));

    let mut renderer = render::Render::new(initial_screen)?;

	// initial_screen.button_map.insert("next_screen", Box::new(||{}));
	
	// (&mut initial_screen).button_map.insert("next_screen", Box::new(|| {renderer.set_screen(1)}));

	let mut button_map_2: HashMap<&str, Box<dyn Fn()>> = HashMap::new();

    let mut new_screen = Screen::new(button_map_2);
    new_screen
        .screen_rows
        .edit_single_row(Text::Button(ButtonText::new(
            "Other Screen",
            width,
            height,
            screen::InsertHorizontalPosition::Right,
            screen::InsertVerticalPosition::Center,
            "last_screen",
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
