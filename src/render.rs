use std::collections::HashMap;
use std::io::{self, stdout, Write};

use crossterm::style::{Color, Stylize};
use crossterm::{cursor, event::KeyCode, execute, queue, terminal};
use rand::Rng;
use regex::Regex;

use crate::chess::ChessPieces;
use crate::screen::{
    ButtonText, InsertHorizontalPosition, InsertVerticalPosition, PlainText, Screen, ScreenRows,
    Text, TextContent,
};

pub struct Render {
    pub screens: Vec<Screen>,
    pub current_screen: usize,
    width: usize,
    height: usize,
    cursor_controller: CursorController,
}

impl Render {
    pub fn new(screen: Screen) -> Result<Self, RenderError> {
        let (term_width, term_height) = terminal::size().unwrap();
        let width = term_width as usize;
        let height = term_height as usize;
        if width < 10 && height < 10 {
            return Err(RenderError::TerminalSizeError(
                "Terminal size is too small!",
            ));
        }
        Ok(Self {
            screens: vec![screen],
            current_screen: 1,
            width,
            height,
            cursor_controller: CursorController::new(width, height),
        })
    }

    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        self.cursor_controller.move_cursor(direction);
    }

    pub fn press_button(&mut self) {
        let cursor_x = self.cursor_controller.cursor_x;
        let cursor_y = self.cursor_controller.cursor_y;
        let current_screen = &mut self.screens[self.current_screen];
        for button in current_screen.screen_rows.buttons[cursor_y].clone() {
            if button.position_x <= cursor_x && button.position_x + button.length > cursor_x {
                if button.on_click == "next_screen" {
                    self.current_screen += 1;
                } else if button.on_click == "last_screen" {
                    self.current_screen -= 1;
                } else if button.on_click == "click_piece" {
                    if let Some(board) = &mut current_screen.game {
                        let piece = board.query_board(cursor_y, cursor_x / 2).0;
                        if piece.symbol != ChessPieces::None {
                            let piece_text = piece.get_symbol().on_dark_yellow().to_string();

                            if let Some(previous_coords) = board.selected_piece {
                                let (previous_piece, previous_piece_text) =
                                    board.query_board(previous_coords.0, previous_coords.1);
                                board.pieces[previous_coords.0][previous_coords.1];

                                current_screen.screen_rows.edit_single_row(Text::new(
                                    previous_piece_text,
                                    previous_piece.file * 2,
                                    previous_piece.rank,
                                    Some("click_piece"),
                                ));

                                let previous_moves = previous_piece.possible_moves();
                                for tile in previous_moves {
                                    let (piece, piece_text) = board.query_board(tile.0, tile.1);

                                    current_screen.screen_rows.edit_single_row(Text::new(
                                        piece_text,
                                        tile.1 * 2,
                                        tile.0,
                                        Some("click_piece"),
                                    ));
                                }
                            }

                            current_screen.screen_rows.edit_single_row(Text::new(
                                piece_text,
                                cursor_x,
                                cursor_y,
                                Some("click_piece"),
                            ));

                            let piece_moves = piece.possible_moves();

                            /* current_screen
                                .screen_rows
                                .clear_row(InsertVerticalPosition::Bottom);
                            current_screen.screen_rows.edit_single_row(Text::Plain(
                                PlainText::new(
                                    format!("Piece moves: {:?}", piece_moves),
                                    current_screen.width,
                                    current_screen.height,
                                    InsertHorizontalPosition::Exact(0),
                                    InsertVerticalPosition::Bottom,
                                ),
                            )); */

                            for tile in piece_moves {
                                let piece = board.pieces[tile.0][tile.1];
                                let piece_text = piece.get_symbol().on_dark_green().to_string();
                                current_screen.screen_rows.edit_single_row(Text::new(
                                    piece_text,
                                    tile.1 * 2,
                                    tile.0,
                                    Some("click_piece"),
                                ));
                            }

                            board.set_selected(cursor_y, cursor_x / 2);
                        }
                    }
                    /* let piece = current_screen.screen_rows.rows[cursor_y][cursor_x].clone();


                    current_screen.screen_rows.edit_single_row(Text::new(
                        piece/* .with(color).on_yellow() */.to_string(),
                        /* piece.on(crossterm::style::Color::Reset).with(crossterm::style::Color::Reset)/* .on_dark_yellow() */.to_string(), */
                        cursor_x,
                        cursor_y,
                        Some("click_piece"),
                    )); */
                } else {
                    let on_click = current_screen
                        .button_map
                        .get_key_value(button.on_click)
                        .unwrap()
                        .1;
                    on_click();
                }
            }
        }
    }

    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(
            self.screens[self.current_screen],
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        self.screens[self.current_screen].compile_screen();
        let cursor_x = self.cursor_controller.cursor_x;
        let cursor_y = self.cursor_controller.cursor_y;
        queue!(
            self.screens[self.current_screen],
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;
        self.screens[self.current_screen].flush()
    }

    pub fn new_screen(&mut self, screen: Screen) {
        self.screens.push(screen);
    }

    pub fn set_screen(&mut self, screen_index: usize) {
        self.current_screen = screen_index;
    }
}

#[derive(Debug)]
pub enum RenderError {
    AddTextError(&'static str),
    TerminalSizeError(&'static str),
}

impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Render error")
    }
}

impl std::error::Error for RenderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

struct CursorController {
    cursor_x: usize,
    cursor_y: usize,
    width: usize,
    height: usize,
}

impl CursorController {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            width,
            height,
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                }
            }
            KeyCode::Down => {
                if self.cursor_y < self.height - 1 {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Left => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_x < self.width - 1 {
                    self.cursor_x += 1;
                }
            }
            _ => unimplemented!(),
        }
    }
}
