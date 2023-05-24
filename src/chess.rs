use crossterm::terminal;

use crate::screen::{self, ButtonText, PlainText, Screen, Text};

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
                        };
                    }
                } else {
                    rank[file_index] = Piece {
                        symbol: ChessPieces::from_fen(char),
                        file: file_index,
                        rank: rank_index,
                    }
                }
            }
            final_board[rank_index] = rank;
        }
        final_board
    }

    pub fn display_board(&self, screen: &mut Screen) {
        // let mut piece_buttons: [[Text; 8]; 8] = [[Text::Button(ButtonText::default()); 8]; 8];
        
        /* let mut row = String::new();
        self.pieces[0].iter().enumerate().map(|(i,piece)| {
            println!("piece: {}",&piece.symbol.to_symbol());
            screen
                .screen_rows
                .edit_single_row(Text::Plain(PlainText::new(
                    piece.symbol.to_symbol().to_string(),
                    width as usize,
                    height as usize,
                    screen::InsertHorizontalPosition::Exact(i),
                    screen::InsertVerticalPosition::Exact(0),
                )))
        }); */

        screen
            .screen_rows
            .edit_single_row(Text::Plain(PlainText::new(
                "stuff thing".to_string(),
                /* self.pieces[0]
                    .map(|piece| piece.symbol.to_symbol())
                    .join(" "), */
                screen.width,
                screen.height,
                screen::InsertHorizontalPosition::Exact(0),
                screen::InsertVerticalPosition::Exact(0),
            )));

        /*         screen
        .screen_rows
        .edit_single_row(Text::Plain(PlainText::new(
            &text,
            width as usize,
            height as usize,
            screen::InsertHorizontalPosition::Exact(0),
            screen::InsertVerticalPosition::Exact(0),
        ))); */
        /* for (j, rank) in self.pieces.iter().enumerate() {
            for (i, piece) in rank.iter().enumerate() {
                piece_buttons[i][j] = Text::Button(ButtonText::new(
                    piece.symbol.to_symbol(),
                    width as usize,
                    height as usize,
                    screen::InsertHorizontalPosition::Exact(0),
                    screen::InsertVerticalPosition::Exact(0),
                    "click_piece",
                ));
            }
        }
        for col in piece_buttons {
            screen.screen_rows.edit_multiple_rows(
                &col,
                0,
                screen::InsertVerticalPosition::Exact(0),
            );
        } */
    }
}

#[derive(Clone, Copy)]
pub struct Piece {
    symbol: ChessPieces,
    file: usize,
    rank: usize,
}

impl Piece {
    fn none() -> Self {
        Self {
            symbol: ChessPieces::None,
            file: 0,
            rank: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub enum ChessPieces {
    None,
    WhiteKing,
    WhiteQueen,
    WhiteRook,
    WhiteBishop,
    WhiteKnight,
    WhitePawn,
    BlackKing,
    BlackQueen,
    BlackRook,
    BlackBishop,
    BlackKnight,
    BlackPawn,
}

impl ChessPieces {
    fn from_fen(piece: char) -> Self {
        match piece {
            // White pieces
            'K' => Self::WhiteKing,
            'Q' => Self::WhiteQueen,
            'B' => Self::WhiteBishop,
            'R' => Self::WhiteRook,
            'N' => Self::WhiteKnight,
            'P' => Self::WhitePawn,
            // Black pieces
            'k' => Self::BlackKing,
            'q' => Self::BlackQueen,
            'b' => Self::BlackBishop,
            'r' => Self::BlackRook,
            'n' => Self::BlackKnight,
            'p' => Self::BlackPawn,
            _ => Self::None,
        }
    }

    fn to_symbol(&self) -> String {
        let symbol = match self {
            ChessPieces::None => "_",
            ChessPieces::WhiteKing => "♔",
            ChessPieces::WhiteQueen => "♕",
            ChessPieces::WhiteRook => "♖",
            ChessPieces::WhiteBishop => "♗",
            ChessPieces::WhiteKnight => "♘",
            ChessPieces::WhitePawn => "♙",
            ChessPieces::BlackKing => "♚",
            ChessPieces::BlackQueen => "♛",
            ChessPieces::BlackRook => "♜",
            ChessPieces::BlackBishop => "♝",
            ChessPieces::BlackKnight => "♞",
            ChessPieces::BlackPawn => "♟︎",
        };
        symbol.to_string()
    }
}
