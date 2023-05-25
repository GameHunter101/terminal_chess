use crossterm::style::Stylize;
use crossterm::terminal;

use crate::screen::{self, ButtonText, PlainText, Screen, Text, TextContent};

pub struct Board {
    pieces: [[Piece; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
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
        let mut board_rows: Vec<Text> = vec![];
        /* for (rank_index, rank) in self.pieces.iter().enumerate() {
            let rank_text = rank.map(|piece| piece.get_symbol()).join(" ");

            board_rows.push(Text::Plain(PlainText::new(
                rank_text,
                screen.width,
                screen.height,
                screen::InsertHorizontalPosition::Exact(0),
                screen::InsertVerticalPosition::Exact(rank_index),
            )));
        } */
        
        /* board_rows.push(Text::Plain(PlainText::new(
            rank_text,
            screen.width,
            screen.height,
            screen::InsertHorizontalPosition::Exact(0),
            screen::InsertVerticalPosition::Exact(0),
        )));
        screen.screen_rows.edit_multiple_rows(
            &board_rows,
            0,
            screen::InsertVerticalPosition::Exact(0),
        ); */
        
        let rank_text = self.pieces[0].map(|piece| piece.get_symbol()).join(" ");
        let text = Text::Plain(PlainText::new(
            rank_text,
            screen.width,
            screen.height,
            screen::InsertHorizontalPosition::Exact(0),
            screen::InsertVerticalPosition::Exact(0),
        ));
        screen
            .screen_rows
            .edit_single_row(text);
    }
}

#[derive(Clone, Copy)]
pub struct Piece {
    symbol: ChessPieces,
    file: usize,
    rank: usize,
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
        match self.white {
            true => symbol.white().to_string(),
            false => symbol.black().on_white().to_string(),
        }
    }
}

#[derive(Clone, Copy)]
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
