use std::collections::HashMap;

use chess::Board;
use crossterm::style::Stylize;
use crossterm::terminal;
use screen::{
    ButtonText, InsertHorizontalPosition, InsertVerticalPosition, PlainText, Screen, Text,
};

mod chess;
mod render;
mod screen;
mod terminal_management;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = terminal::size().unwrap();
    let width = width as usize;
    let height = height as usize;
    let button_map: HashMap<&str, Box<dyn Fn()>> = HashMap::new();
    let mut initial_screen = Screen::new(button_map, None);
    initial_screen.screen_rows.edit_multiple_rows(
        &PlainText::from_multi_lines(
r#" _______                       __               __      ______ __                      
|_     _|.-----.----.--------.|__|.-----.---.-.|  |    |      |  |--.-----.-----.-----.
  |   |  |  -__|   _|        ||  ||     |  _  ||  |    |   ---|     |  -__|__ --|__ --|
  |___|  |_____|__| |__|__|__||__||__|__|___._||__|    |______|__|__|_____|_____|_____|"#.to_string(),
            width,
            screen::InsertHorizontalPosition::Center,
        ),
        0,
        screen::InsertVerticalPosition::Exact(3),
    );

    initial_screen
        .screen_rows
        .edit_single_row(Text::Button(ButtonText::new(
            "Play Game".red().slow_blink().to_string(),
            width,
            height,
            InsertHorizontalPosition::Center,
            InsertVerticalPosition::Exact(10),
            "next_screen",
        )));

    initial_screen
        .screen_rows
        .edit_single_row(Text::Plain(PlainText::new(
            "Created by Lior Carmeli for APCSP".dark_red().to_string(),
            width,
            height,
            InsertHorizontalPosition::Center,
            InsertVerticalPosition::Exact(12),
        )));

    let mut renderer = render::Render::new(initial_screen)?;

    let chess_game = Board::new();

    let game_button_map: HashMap<&str, Box<dyn Fn()>> = HashMap::new();

    let mut game_screen = Screen::new(game_button_map, Some(chess_game));

    let board_rows = chess_game.display_board();

    for row in board_rows {
        for piece in row {
            game_screen.screen_rows.edit_single_row(piece);
        }
    }

    game_screen
        .screen_rows
        .edit_single_row(Text::Plain(PlainText::new(
            "White's turn".to_string(),
            width,
            height,
            InsertHorizontalPosition::Exact(0),
            InsertVerticalPosition::Exact(10),
        )));

    game_screen
        .screen_rows
        .edit_single_row(Text::Button(ButtonText::new(
            "<= HOME".to_string(),
            width,
            height,
            InsertHorizontalPosition::Exact(0),
            InsertVerticalPosition::Center,
            "last_screen",
        )));

    renderer.new_screen(game_screen);

    let victory_button_map: HashMap<&str, Box<dyn Fn()>> = HashMap::new();

    let mut victory_screen = Screen::new(victory_button_map, None);

    victory_screen.screen_rows.edit_multiple_rows(
        &vec![
            Text::Plain(PlainText::new(
                "Congratulations, you won!".to_string(),
                width,
                height,
                InsertHorizontalPosition::Center,
                InsertVerticalPosition::Center,
            )),
            Text::Button(ButtonText::new(
                "Reset".to_string(),
                width,
                height,
                InsertHorizontalPosition::Center,
                InsertVerticalPosition::Center,
                "reset_game",
            )),
        ],
        1,
        InsertVerticalPosition::Center,
    );

    renderer.new_screen(victory_screen);

    let mut terminal = terminal_management::Terminal::new(renderer);

    terminal::enable_raw_mode().expect("Could not turn on raw mode");
    while terminal.run().unwrap() {}
    Ok(())
}
