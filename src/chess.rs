use crossterm::style::Stylize;
use crossterm::terminal;

use crate::screen::{self, ButtonText, PlainText, Screen, Text, TextContent};

pub struct Board {
    pieces: [[Piece; 8]; 8],
    selected_piece: Option<(usize, usize)>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            selected_piece: None,
        }
    }

    // Converts a given Forsyth-Edwards Notation string to a chess board
    fn from_fen(fen: &'static str) -> [[Piece; 8]; 8] {
        let piece_data = fen.split(" ").collect::<Vec<_>>()[0];
        let lines = piece_data.split("/");
        let mut final_board = [[Piece::none(); 8]; 8];
        for (rank_index, line) in lines.enumerate() {
            let mut rank: [Piece; 8] = [Piece::none(); 8];
            for (file_index, char) in line.chars().enumerate() {
                if let Some(blank_count) = char.to_digit(10) {
                    for i in 0..blank_count {
                        rank[file_index + i as usize] = Piece {
                            symbol: ChessPieces::None,
                            file: file_index + i as usize,
                            rank: rank_index,
                            white: true,
                        };
                    }
                } else {
                    rank[file_index] = Piece {
                        symbol: ChessPieces::from_fen(char),
                        file: file_index,
                        rank: rank_index,
                        white: char.is_uppercase(),
                    };
                }
            }
            final_board[rank_index] = rank;
        }
        final_board
    }

    pub fn display_board(&self, screen: &mut Screen) {
        let mut board_rows: Vec<Vec<Text>> = vec![vec![]; 8];
        //     screen.button_map.insert(
        //         "click_piece",
        //         Box::new(|| {
        // let piece_x = self.selected_piece.unwrap().0;
        // let piece_y = self.selected_piece.unwrap().1;
        //             let piece_string = &board_rows[piece_x].;
        //         }),
        //     );
        for (rank_index, rank) in self.pieces.iter().enumerate() {
            for (file_index, piece) in rank.iter().enumerate() {
                let checker_index = (piece.file + piece.rank) % 2;
                let piece_text = if checker_index == 0 {
                    piece.get_symbol()
                } else {
                    piece.get_symbol() /* .on_dark_grey().to_string() */
                };
				// println!("{}",piece_text.clone().on_red());
                // dbg!(&piece_text);
                board_rows[rank_index].push(Text::Plain(PlainText::new(
                    format!("{}",piece_text.on_red()),
                    screen.width,
                    screen.height,
                    screen::InsertHorizontalPosition::Exact(file_index),
                    screen::InsertVerticalPosition::Exact(rank_index),
                    // "click_piece",
                )));
            }
        }
        // panic!("test");

        /* board_rows.push(Text::Plain(PlainText::new(
            rank_text,
            screen.width,
            screen.height,
            screen::InsertHorizontalPosition::Exact(0),
            screen::InsertVerticalPosition::Exact(0),
        ))); */
        // dbg!(&board_rows);
		
   //      for row in board_rows {
			// for piece in row {
			// 	screen.screen_rows.edit_single_row(piece);
			// }
            /* screen.screen_rows.edit_multiple_rows(
                &row,
                0,
                screen::InsertVerticalPosition::Exact(0),
            ); */
        // }
    }
}

#[derive(Clone, Copy)]
pub struct Piece {
    symbol: ChessPieces,
    pub file: usize,
    pub rank: usize,
    white: bool,
}

impl Piece {
    fn none() -> Self {
        Self {
            symbol: ChessPieces::None,
            file: 0,
            rank: 0,
            white: true,
        }
    }

    fn get_symbol(&self) -> String {
        let symbol = self.symbol.to_symbol();
        /* match self.white {
            true => {
                if self.symbol == ChessPieces::None {
                    symbol.hidden().to_string()
                } else {
                    symbol.white().to_string()
                }
            }
            false => symbol.red().to_string(),
        } */
        symbol
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ChessPieces {
    None,
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl ChessPieces {
    fn from_fen(piece: char) -> Self {
        match piece {
            // White pieces
            'K' => Self::King,
            'Q' => Self::Queen,
            'B' => Self::Bishop,
            'R' => Self::Rook,
            'N' => Self::Knight,
            'P' => Self::Pawn,
            // Black pieces
            'k' => Self::King,
            'q' => Self::Queen,
            'b' => Self::Bishop,
            'r' => Self::Rook,
            'n' => Self::Knight,
            'p' => Self::Pawn,
            _ => Self::None,
        }
    }

    fn to_symbol(&self) -> String {
        let symbol = match self {
            ChessPieces::None => "_",
            ChessPieces::King => "♔",
            ChessPieces::Queen => "♕",
            ChessPieces::Rook => "♖",
            ChessPieces::Bishop => "♗",
            ChessPieces::Knight => "♘",
            ChessPieces::Pawn => "♙",
        };
        symbol.to_string()
    }
}
