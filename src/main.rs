#![allow(unused)]

use std::collections::HashMap;
use std::io;

use chess::{Board, Piece};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::style::Stylize;
use crossterm::terminal;
use render::Render;
use screen::{ButtonText, PlainText, Screen, Text};

mod chess;
mod render;
mod screen;
mod terminal_management;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = terminal::size().unwrap();
    let width = width as usize;
    let height = height as usize;
    let mut button_map: HashMap<&str, Box<dyn Fn()>> = HashMap::new();
    let mut initial_screen = Screen::new(button_map, None);
    /* initial_screen.screen_rows.edit_multiple_rows(
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
    ); */

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

    let mut renderer = render::Render::new(initial_screen)?;

    let chess_game = Board::new();

    let mut game_button_map: HashMap<&str, Box<dyn Fn()>> = HashMap::new();

    let mut game_screen = Screen::new(game_button_map, Some(chess_game));

    let board_rows = chess_game.display_board(false);

	for row in board_rows {
            for piece in row {
                game_screen.screen_rows.edit_single_row(piece);
            }
	}

    game_screen
        .screen_rows
        .edit_single_row(Text::Button(ButtonText::new(
            "Other Screen".to_string(),
            width,
            height,
            screen::InsertHorizontalPosition::Right,
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

fn query_board(board: &Board, pos_x: usize, pos_y: usize) -> Piece {
    board.pieces[pos_y][pos_x]
}
