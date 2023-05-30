use crossterm::style::Stylize;
use crossterm::terminal;

use crate::screen::{self, ButtonText, PlainText, Screen, Text, TextContent};

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub pieces: [[Piece; 8]; 8],
    pub selected_piece: Option<(usize, usize)>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: Board::from_fen(
                "rnbqkbnr/pppppppp/8/rnbqkbp1/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ),
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

        for (rank_index, rank) in self.pieces.iter().enumerate() {
            for (file_index, piece) in rank.iter().enumerate() {
                let checker_index = (piece.file + piece.rank) % 2;
                let piece_text = if checker_index == 0 {
                    piece.get_symbol()
                } else {
                    piece
                        .get_symbol()
                        .on(crossterm::style::Color::AnsiValue(237))
                        .to_string()
                };

                board_rows[rank_index].push(Text::Button(ButtonText::new(
                    piece_text,
                    screen.width,
                    screen.height,
                    screen::InsertHorizontalPosition::Exact(file_index * 2),
                    screen::InsertVerticalPosition::Exact(rank_index),
                    "click_piece",
                )));
            }
        }

        for row in board_rows {
            for piece in row {
                screen.screen_rows.edit_single_row(piece);
            }
        }
    }

    pub fn set_selected(&mut self, rank: usize, file: usize) -> Self {
        self.selected_piece = Some((rank, file));
        *self
    }

    pub fn query_board(&self, rank: usize, file: usize) -> (Piece, String) {
        let piece = self.pieces[rank][file];
        let piece_checker = (rank + file) % 2;
        let piece_text = if piece_checker == 0 {
            piece.get_symbol()
        } else {
            piece
                .get_symbol()
                .on(crossterm::style::Color::AnsiValue(237))
                .to_string()
        };
        (piece, piece_text)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub symbol: ChessPieces,
    pub file: usize,
    pub rank: usize,
    pub white: bool,
}

impl Piece {
    pub fn none() -> Self {
        Self {
            symbol: ChessPieces::None,
            file: 0,
            rank: 0,
            white: true,
        }
    }

    pub fn get_symbol(&self) -> String {
        let symbol = self.symbol.to_symbol();
        match self.white {
            true => {
                if self.symbol == ChessPieces::None {
                    symbol.hidden().to_string()
                } else {
                    symbol.white().to_string()
                }
            }
            false => symbol.red().to_string(),
        }
    }

    pub fn possible_moves(&self) -> Vec<(usize, usize)> {
        match self.symbol {
            ChessPieces::Pawn => {
                let mut moves = vec![];

                if self.white {
                    moves.push((self.rank - 1, self.file));
                    if self.rank == 6 {
                        moves.push((self.rank - 2, self.file));
                    }
                } else {
                    moves.push((self.rank + 1, self.file));
                    if self.rank == 1 {
                        moves.push((self.rank + 2, self.file));
                    }
                }

                moves
            }
            ChessPieces::Rook => {
                let mut moves = vec![];

                for i in 0..8 {
                    if i != self.file {
                        moves.push((self.rank, i));
                    }
                    if i != self.rank {
                        moves.push((i, self.file));
                    }
                }

                moves
            }
            ChessPieces::Knight => {
                let mut moves = vec![];
                let mut offsets: Vec<(i32, i32)> = vec![];
                offsets.push((2, 1));
                offsets.push((2, -1));
                offsets.push((-2, 1));
                offsets.push((-2, -1));

                offsets.push((1, 2));
                offsets.push((1, -2));
                offsets.push((-1, 2));
                offsets.push((-1, -2));

                for (off_x, off_y) in offsets {
                    let file = self.file as i32 + off_x;
                    let rank = self.rank as i32 + off_y;
                    if file >= 0 && rank >= 0 && file < 8 && rank < 8 {
                        moves.push((rank as usize, file as usize));
                    }
                }

                moves
            }
            ChessPieces::Bishop => {
                let mut moves = vec![];

                for i in -8_i32..8 {
                    let move_x = self.file as i32 + i;
                    let move_y_1 = self.rank as i32 + i;
                    let move_y_2 = self.rank as i32 - i;
                    if move_x >= 0
                        && move_y_1 >= 0
                        && move_x < 8
                        && move_y_1 < 8
                        && move_x != self.file as i32
                    {
                        moves.push((move_y_1 as usize, move_x as usize));
                    }
                    if move_x >= 0
                        && move_y_2 >= 0
                        && move_x < 8
                        && move_y_2 < 8
                        && move_x != self.file as i32
                    {
                        moves.push((move_y_2 as usize, move_x as usize));
                    }
                }

                moves
            }
            ChessPieces::Queen => {
                let mut moves = vec![];

                for i in 0..8 {
                    if i != self.file {
                        moves.push((self.rank, i));
                    }
                    if i != self.rank {
                        moves.push((i, self.file));
                    }
                }

                for i in -8_i32..8 {
                    let move_x = self.file as i32 + i;
                    let move_y_1 = self.rank as i32 + i;
                    let move_y_2 = self.rank as i32 - i;
                    if move_x >= 0
                        && move_y_1 >= 0
                        && move_x < 8
                        && move_y_1 < 8
                        && move_x != self.file as i32
                    {
                        moves.push((move_y_1 as usize, move_x as usize));
                    }
                    if move_x >= 0
                        && move_y_2 >= 0
                        && move_x < 8
                        && move_y_2 < 8
                        && move_x != self.file as i32
                    {
                        moves.push((move_y_2 as usize, move_x as usize));
                    }
                }

                moves
            }
            ChessPieces::King => {
                let mut moves = vec![];
                let mut offsets = vec![];
                offsets.push((-1, -1));
                offsets.push((-1, 0));
                offsets.push((-1, 1));

                offsets.push((1, -1));
                offsets.push((1, 0));
                offsets.push((1, 1));

                offsets.push((0, 1));
                offsets.push((0, -1));

                for (off_x, off_y) in offsets {
                    let file = self.file as i32 + off_x;
                    let rank = self.rank as i32 + off_y;
                    if file >= 0 && rank >= 0 && file < 8 && rank < 8 {
                        moves.push((rank as usize, file as usize));
                    }
                }
                moves
            }
            ChessPieces::None => {
                vec![]
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
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
