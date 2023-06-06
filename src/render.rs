use std::collections::HashMap;
use std::io::{self, stdout, Write};

use crossterm::style::{Color, Stylize};
use crossterm::{cursor, event::KeyCode, execute, queue, terminal};
use rand::Rng;
use regex::Regex;

use crate::chess::{Board, ChessPieces};
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

    pub fn move_cursor_half(&mut self, direction: KeyCode) {
        self.cursor_controller.move_cursor_half(direction);
    }

    pub fn move_cursor_far(&mut self, direction: KeyCode) {
        self.cursor_controller.move_cursor_far(direction);
    }

    pub fn press_button(&mut self) {
        let cursor_x = self.cursor_controller.cursor_x;
        let cursor_y = self.cursor_controller.cursor_y;
        let current_screen = &mut self.screens[self.current_screen];
        if let Some(board) = &mut current_screen.game {
            if cursor_x < 16 && cursor_y < 8 {
                let board_rows = board.display_board(true);

                for row in board_rows {
                    for piece in row {
                        current_screen.screen_rows.edit_single_row(piece);
                    }
                }

                let piece = board.query_board(cursor_y, cursor_x / 2).0;

                if piece.symbol != ChessPieces::None
                    && piece.white != board.white_move
                    && !board.moving
                {
                    return;
                }

                if board.moving {
                    let selected_piece_coords = board.selected_piece.unwrap();
                    let selected_piece = board
                        .query_board(selected_piece_coords.0, selected_piece_coords.1)
                        .0;
                    let did_win = board.move_piece(selected_piece, cursor_y, cursor_x / 2);

                    let board_rows = board.display_board(true);

                    for row in board_rows {
                        for piece in row {
                            current_screen.screen_rows.edit_single_row(piece);
                        }
                    }

                    if did_win {
                        /* self.screens[2]
                        .screen_rows
                        .edit_single_row(Text::Plain(PlainText::new(
                            format!(
                                "Congratulations, {} won!",
                                if board.white_move { "white" } else { "black" }
                            ),
                            self.width,
                            self.height,
                            InsertHorizontalPosition::Center,
                            InsertVerticalPosition::Center,
                        ))); */
                        current_screen.game = Some(Board::new());
                        let board_rows = current_screen.game.unwrap().display_board(true);

                        for row in board_rows {
                            for piece in row {
                                current_screen.screen_rows.edit_single_row(piece);
                            }
                        }
                        self.current_screen += 1;
                    }

                    return;
                }

                let piece_moves = board.filter_possible_moves(piece);

                for tile in piece_moves {
                    let piece = board.pieces[tile.0][tile.1];
                    let piece_text = piece.get_symbol().on_dark_green().to_string();
                    current_screen.screen_rows.edit_single_row(Text::new(
                        piece_text,
                        tile.1 * 2,
                        tile.0,
                        None,
                    ));
                }
                board.moving = true;

                board.set_selected(cursor_y, cursor_x / 2);
            }
        }
        for button in current_screen.screen_rows.buttons[cursor_y].clone() {
            if button.position_x <= cursor_x && button.position_x + button.length > cursor_x {
                if button.on_click == "next_screen" {
                    self.current_screen += 1;
                } else if button.on_click == "last_screen" {
                    self.current_screen -= 1;
                } else if button.on_click == "reset_game" {
                    self.current_screen = 0;
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

    pub fn move_cursor_half(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up => {
                self.cursor_y = self.height / 2;
            }
            KeyCode::Down => {
                self.cursor_y = self.height / 2;
            }
            KeyCode::Left => {
                self.cursor_x = self.width / 2;
            }
            KeyCode::Right => {
                self.cursor_x = self.width / 2;
            }
            _ => unimplemented!(),
        }
    }

    pub fn move_cursor_far(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up => {
                self.cursor_y = 0;
            }
            KeyCode::Down => {
                self.cursor_y = self.height - 1;
            }
            KeyCode::Left => {
                self.cursor_x = 0;
            }
            KeyCode::Right => {
                self.cursor_x = self.width - 1;
            }
            _ => unimplemented!(),
        }
    }
}
