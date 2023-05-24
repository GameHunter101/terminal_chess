#![allow(unused)]

use std::collections::HashMap;
use std::io;

use chess::Board;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal;
use render::Render;
use screen::{ButtonText, PlainText, Screen, Text};

mod render;
mod screen;
mod terminal_management;
mod chess;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = terminal::size().unwrap();
    let width = width as usize;
    let height = height as usize;
    let mut button_map: HashMap<&str, Box<dyn Fn()>> = HashMap::new();
    let mut initial_screen = Screen::new(button_map);
    initial_screen.screen_rows.edit_multiple_rows(
        &PlainText::from_multi_lines(
r#" _______                       __               __      ______ __                      
|_     _|.-----.----.--------.|__|.-----.---.-.|  |    |      |  |--.-----.-----.-----.
  |   |  |  -__|   _|        ||  ||     |  _  ||  |    |   ---|     |  -__|__ --|__ --|
  |___|  |_____|__| |__|__|__||__||__|__|___._||__|    |______|__|__|_____|_____|_____|"#.to_string(),
            width,
            height,
            screen::InsertHorizontalPosition::Center,
        ),
        0,
        screen::InsertVerticalPosition::Exact(3),
    );

    initial_screen
        .screen_rows
        .edit_single_row(Text::Button(ButtonText::new(
            "Play Game".to_string(),
            width,
            height,
            screen::InsertHorizontalPosition::Center,
            screen::InsertVerticalPosition::Exact(10),
            "next_screen",
        )));

    // (&mut initial_screen).button_map.insert("next_screen", Box::new(|| {crossterm::queue!(io::stdout(),crossterm::style::SetBackgroundColor(crossterm::style::Color::Rgb { r: 50, g: 0, b: 50 }));}));

    let mut renderer = render::Render::new(initial_screen)?;

    // initial_screen.button_map.insert("next_screen", Box::new(||{}));

    // (&mut initial_screen).button_map.insert("next_screen", Box::new(|| {renderer.set_screen(1)}));

    let mut game_button_map: HashMap<&str, Box<dyn Fn()>> = HashMap::new();

    let mut game_screen = Screen::new(game_button_map);

    let chess_game = Board::new();

    chess_game.display_board(&mut game_screen);

    game_screen
        .screen_rows
        .edit_single_row(Text::Button(ButtonText::new(
            "Other Screen".to_string(),
            width,
            height,
            screen::InsertHorizontalPosition::Exact(width-12),
            screen::InsertVerticalPosition::Center,
            "last_screen",
        )));
    renderer.new_screen(game_screen);
    let mut terminal = terminal_management::Terminal::new(renderer);

    // TODO:
    // Find a way to set foreground and background color for each text

    terminal::enable_raw_mode().expect("Could not turn on raw mode");
    while terminal.run().unwrap() {}
    Ok(())
}
