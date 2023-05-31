use std::ops::Add;

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
                "rnbqkbnr/pppppppp/7P/knbqr3/7r/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
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
            let mut new_line: String = line.to_string();
            for (i, char) in line.char_indices() {
                if let Some(blank_count) = char.to_digit(10) {
                    let split = line.split_at(i);
                    new_line = split.0.to_string();
                    new_line.push_str(&"T".repeat(blank_count as usize));
                    let end_line: String = split.1.chars().collect::<Vec<char>>()[1..]
                        .iter()
                        .map(|c| c.to_string())
                        .collect();
                    new_line.push_str(&end_line);
                }
            }

            for (file_index, char) in new_line.char_indices() {
                rank[file_index] = Piece {
                    symbol: ChessPieces::from_fen(char),
                    file: file_index,
                    rank: rank_index,
                    white: char.is_uppercase(),
                };
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

    pub fn closest_tile_left(&self, rank: usize, file: usize) -> usize {
        let piece_rank = self.pieces[rank];

        let mut whole_rank_left = &mut piece_rank.clone()[..file];
        whole_rank_left.reverse();
        let mut dist_left = whole_rank_left.len();
        for (dist, piece) in whole_rank_left.iter().enumerate() {
            if piece.symbol != ChessPieces::None {
                dist_left = dist;
                break;
            }
        }
		let last_piece = whole_rank_left[dist_left];
		

        dist_left
    }

    pub fn closest_tile_right(&self, rank: usize, file: usize) -> usize {
        let piece_rank = self.pieces[rank];

        let mut whole_rank_right = &mut piece_rank.clone()[file + 1..];
        let mut dist_right = whole_rank_right.len();
        for (dist, piece) in whole_rank_right.iter().enumerate() {
            if piece.symbol != ChessPieces::None {
                dist_right = dist;
                break;
            }
        }

        dist_right
    }
    pub fn closest_tile_up(&self, rank: usize, file: usize) -> usize {
        let piece_file = self.pieces.map(|rank| rank[file]);

        let mut whole_file_up = &mut piece_file.clone()[..rank];
        whole_file_up.reverse();
        let mut dist_up = whole_file_up.len();
        for (dist, piece) in whole_file_up.iter().enumerate() {
            if piece.symbol != ChessPieces::None {
                dist_up = dist;
                break;
            }
        }

        dist_up
    }

    pub fn closest_tile_down(&self, rank: usize, file: usize) -> usize {
        let piece_file = self.pieces.map(|rank| rank[file]);

        let mut whole_file_up = &mut piece_file.clone()[rank + 1..];
        let mut dist_up = whole_file_up.len();
        for (dist, piece) in whole_file_up.iter().enumerate() {
            if piece.symbol != ChessPieces::None {
                dist_up = dist;
                break;
            }
        }

        dist_up
    }

    pub fn possible_moves(&self, origin_piece: Piece) -> Vec<(usize, usize)> {
        match origin_piece.symbol {
            ChessPieces::Pawn => {
                let mut moves = vec![];

                if origin_piece.white {
                    moves.push((origin_piece.rank - 1, origin_piece.file));
                    if origin_piece.rank == 6 {
                        moves.push((origin_piece.rank - 2, origin_piece.file));
                    }

                    if origin_piece.rank > 0 {
                        if origin_piece.file > 0 {
                            let query_left_diagonal_tile = self
                                .query_board(origin_piece.rank - 1, origin_piece.file - 1)
                                .0;
                            if query_left_diagonal_tile.white != origin_piece.white
                                && query_left_diagonal_tile.symbol != ChessPieces::None
                            {
                                moves.push((origin_piece.rank - 1, origin_piece.file - 1));
                            }
                        }
                        if origin_piece.file < 7 {
                            let query_right_diagonal_tile = self
                                .query_board(origin_piece.rank - 1, origin_piece.file + 1)
                                .0;
                            if query_right_diagonal_tile.white != origin_piece.white
                                && query_right_diagonal_tile.symbol != ChessPieces::None
                            {
                                moves.push((origin_piece.rank - 1, origin_piece.file + 1));
                            }
                        }
                    }
                } else {
                    moves.push((origin_piece.rank + 1, origin_piece.file));
                    if origin_piece.rank == 1 {
                        moves.push((origin_piece.rank + 2, origin_piece.file));
                    }

                    if origin_piece.rank < 7 {
                        if origin_piece.file > 0 {
                            let query_left_diagonal_tile = self
                                .query_board(origin_piece.rank + 1, origin_piece.file - 1)
                                .0;
                            if query_left_diagonal_tile.white != origin_piece.white
                                && query_left_diagonal_tile.symbol != ChessPieces::None
                            {
                                moves.push((origin_piece.rank + 1, origin_piece.file - 1));
                            }
                        }
                        if origin_piece.file < 7 {
                            let query_right_diagonal_tile = self
                                .query_board(origin_piece.rank + 1, origin_piece.file + 1)
                                .0;
                            if query_right_diagonal_tile.white != origin_piece.white
                                && query_right_diagonal_tile.symbol != ChessPieces::None
                            {
                                moves.push((origin_piece.rank + 1, origin_piece.file + 1));
                            }
                        }
                    }
                }

                moves
            }
            ChessPieces::Rook => {
                let mut moves = vec![];

                let dist_left = self.closest_tile_left(origin_piece.rank, origin_piece.file);
                let dist_right = self.closest_tile_right(origin_piece.rank, origin_piece.file);
                let dist_up = self.closest_tile_up(origin_piece.rank, origin_piece.file);
                let dist_down = self.closest_tile_down(origin_piece.rank, origin_piece.file);

                for i in 0..dist_left {
                    moves.push((origin_piece.rank, origin_piece.file - i - 1));
                }

                for i in 0..dist_right {
                    moves.push((origin_piece.rank, origin_piece.file + i + 1));
                }

                for i in 0..dist_up {
                    moves.push((origin_piece.rank - i - 1, origin_piece.file));
                }

				for i in 0..dist_down {
                    moves.push((origin_piece.rank + i + 1, origin_piece.file));
                }

                // for i in 0..8 {
                //     if i != origin_piece.file {
                //         moves.push((origin_piece.rank, i));
                //     }
                //     if i != origin_piece.rank {
                //         moves.push((i, origin_piece.file));
                //     }
                // }

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
                    let file = origin_piece.file as i32 + off_x;
                    let rank = origin_piece.rank as i32 + off_y;
                    if file >= 0 && rank >= 0 && file < 8 && rank < 8 {
                        moves.push((rank as usize, file as usize));
                    }
                }

                moves
            }
            ChessPieces::Bishop => {
                let mut moves = vec![];

                for i in -8_i32..8 {
                    let move_x = origin_piece.file as i32 + i;
                    let move_y_1 = origin_piece.rank as i32 + i;
                    let move_y_2 = origin_piece.rank as i32 - i;
                    if move_x >= 0
                        && move_y_1 >= 0
                        && move_x < 8
                        && move_y_1 < 8
                        && move_x != origin_piece.file as i32
                    {
                        moves.push((move_y_1 as usize, move_x as usize));
                    }
                    if move_x >= 0
                        && move_y_2 >= 0
                        && move_x < 8
                        && move_y_2 < 8
                        && move_x != origin_piece.file as i32
                    {
                        moves.push((move_y_2 as usize, move_x as usize));
                    }
                }

                moves
            }
            ChessPieces::Queen => {
                let mut moves = vec![];

                let dist_left = self.closest_tile_left(origin_piece.rank, origin_piece.file);
                let dist_right = self.closest_tile_right(origin_piece.rank, origin_piece.file);
                let dist_up = self.closest_tile_up(origin_piece.rank, origin_piece.file);
                let dist_down = self.closest_tile_down(origin_piece.rank, origin_piece.file);

                for i in 0..dist_left {
                    moves.push((origin_piece.rank, origin_piece.file - i - 1));
                }

                for i in 0..dist_right {
                    moves.push((origin_piece.rank, origin_piece.file + i + 1));
                }

                for i in 0..dist_up {
                    moves.push((origin_piece.rank - i - 1, origin_piece.file));
                }

				for i in 0..dist_down {
                    moves.push((origin_piece.rank + i + 1, origin_piece.file));
                }

                for i in -8_i32..8 {
                    let move_x = origin_piece.file as i32 + i;
                    let move_y_1 = origin_piece.rank as i32 + i;
                    let move_y_2 = origin_piece.rank as i32 - i;
                    if move_x >= 0
                        && move_y_1 >= 0
                        && move_x < 8
                        && move_y_1 < 8
                        && move_x != origin_piece.file as i32
                    {
                        moves.push((move_y_1 as usize, move_x as usize));
                    }
                    if move_x >= 0
                        && move_y_2 >= 0
                        && move_x < 8
                        && move_y_2 < 8
                        && move_x != origin_piece.file as i32
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
                    let file = origin_piece.file as i32 + off_x;
                    let rank = origin_piece.rank as i32 + off_y;
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

    pub fn filter_possible_moves(&self, origin_piece: Piece) -> Vec<(usize, usize)> {
        // let mut filtered_moves = vec![];
        let possible_moves = self.possible_moves(origin_piece);
        /* for tile in possible_moves {
            let piece = self.query_board(tile.0, tile.1).0;
            if piece.symbol == ChessPieces::None || piece.white != origin_piece.white {
                filtered_moves.push(tile);
            }
        }
        filtered_moves */
        possible_moves
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
