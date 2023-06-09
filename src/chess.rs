use crossterm::style::Stylize;

use crate::screen::Text;

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub pieces: [[Piece; 8]; 8],
    pub selected_piece: Option<(usize, usize)>,
    pub white_move: bool,
    pub moving: bool,
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            selected_piece: None,
            white_move: true,
            moving: false,
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

    // Returns the baord as a 2d vector of Text objects
    pub fn display_board(&self) -> Vec<Vec<Text>> {
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

                board_rows[rank_index].push(Text::new(
                    piece_text,
                    file_index * 2,
                    rank_index,
                    None,
                ));
            }
        }

        board_rows
    }

    // Sets the selected piece for use in piece movement
    pub fn set_selected(&mut self, rank: usize, file: usize) -> Self {
        self.selected_piece = Some((rank, file));
        *self
    }

    // Returns the piece at the given position and its printable text, including ANSI sequences
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

    // Gets the valid positions to the left of a piece
    pub fn valid_positions_left(
        &self,
        rank: usize,
        file: usize,
        white: bool,
    ) -> Vec<(usize, usize)> {
        let mut left_moves = vec![];
        let piece_rank = self.pieces[rank];

        let whole_rank_left = &mut piece_rank.clone()[..file];
        whole_rank_left.reverse();
        let mut dist_left = whole_rank_left.len();
        for (dist, piece) in whole_rank_left.iter().enumerate() {
            if piece.symbol != ChessPieces::None {
                dist_left = dist;
                break;
            }
        }
        if whole_rank_left.len() > 0 && dist_left < whole_rank_left.len() {
            let last_piece = whole_rank_left[dist_left];
            if last_piece.white != white {
                dist_left += 1;
            }
        }

        for i in 0..dist_left {
            left_moves.push((rank, file - i - 1));
        }

        left_moves
    }

    // Gets the valid positions to the right of a piece
    pub fn valid_positions_right(
        &self,
        rank: usize,
        file: usize,
        white: bool,
    ) -> Vec<(usize, usize)> {
        let mut right_moves = vec![];
        let piece_rank = self.pieces[rank];

        let whole_rank_right = &mut piece_rank.clone()[file + 1..];
        let mut dist_right = whole_rank_right.len();
        for (dist, piece) in whole_rank_right.iter().enumerate() {
            if piece.symbol != ChessPieces::None {
                dist_right = dist;
                break;
            }
        }

        if whole_rank_right.len() > 0 && dist_right < whole_rank_right.len() {
            let last_piece = whole_rank_right[dist_right];
            if last_piece.white != white {
                dist_right += 1;
            }
        }

        for i in 0..dist_right {
            right_moves.push((rank, file + i + 1));
        }

        right_moves
    }

    // Gets the valid positions above the piece
    pub fn valid_positions_up(&self, rank: usize, file: usize, white: bool) -> Vec<(usize, usize)> {
        let mut up_moves = vec![];
        let piece_file = self.pieces.map(|rank| rank[file]);

        let whole_file_up = &mut piece_file.clone()[..rank];
        whole_file_up.reverse();
        let mut dist_up = whole_file_up.len();
        for (dist, piece) in whole_file_up.iter().enumerate() {
            if piece.symbol != ChessPieces::None {
                dist_up = dist;
                break;
            }
        }

        if whole_file_up.len() > 0 && dist_up < whole_file_up.len() {
            let last_piece = whole_file_up[dist_up];
            if last_piece.white != white {
                dist_up += 1;
            }
        }

        for i in 0..dist_up {
            up_moves.push((rank - i - 1, file));
        }

        up_moves
    }

    // Gets the valid positions below the piece
    pub fn valid_positions_down(
        &self,
        rank: usize,
        file: usize,
        white: bool,
    ) -> Vec<(usize, usize)> {
        let mut down_moves = vec![];
        let piece_file = self.pieces.map(|rank| rank[file]);

        let whole_file_down = &mut piece_file.clone()[rank + 1..];
        let mut dist_down = whole_file_down.len();
        for (dist, piece) in whole_file_down.iter().enumerate() {
            if piece.symbol != ChessPieces::None {
                dist_down = dist;
                break;
            }
        }

        if whole_file_down.len() > 0 && dist_down < whole_file_down.len() {
            let last_piece = whole_file_down[dist_down];
            if last_piece.white != white {
                dist_down += 1;
            }
        }

        for i in 0..dist_down {
            down_moves.push((rank + i + 1, file));
        }

        down_moves
    }

    // Gets the valid positions in a y=x line going through the piece
    pub fn valid_positions_diagonal_positive(
        &self,
        rank: usize,
        file: usize,
        white: bool,
    ) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        let mut diagonal_top: Vec<(usize, usize)> = vec![];
        let mut diagonal_bottom: Vec<(usize, usize)> = vec![];
        for (i, _) in self.pieces.iter().enumerate() {
            let offset = rank as i32 - i as i32;
            let diagonal_file = file as i32 + offset;
            if diagonal_file >= 0 && diagonal_file < 8 {
                if i < rank {
                    diagonal_top.push((i, diagonal_file as usize));
                } else if i > rank {
                    diagonal_bottom.push((i, diagonal_file as usize));
                }
            }
        }

        diagonal_top.reverse();
        for tile in diagonal_top {
            let query = self.query_board(tile.0, tile.1).0;
            if query.symbol == ChessPieces::None {
                moves.push(tile);
            } else {
                if query.white != white {
                    moves.push(tile);
                }
                break;
            }
        }

        for tile in diagonal_bottom {
            let query = self.query_board(tile.0, tile.1).0;
            if query.symbol == ChessPieces::None {
                moves.push(tile);
            } else {
                if query.white != white {
                    moves.push(tile);
                }
                break;
            }
        }
        moves
    }

    // Gets the valid pieces in a y=-x line going through the piece
    pub fn piece_moves_diagonal_negative(
        &self,
        rank: usize,
        file: usize,
        white: bool,
    ) -> Vec<(usize, usize)> {
        let mut moves = vec![];
        let mut diagonal_top: Vec<(usize, usize)> = vec![];
        let mut diagonal_bottom: Vec<(usize, usize)> = vec![];

        let offset = (file as i32 - 3) + (3 - rank as i32);
        for (i, _) in self.pieces.iter().enumerate() {
            let tile_file = i as i32 + offset;
            let tile_rank = i;
            if tile_file >= 0 && tile_file < 8 {
                if tile_rank < rank {
                    diagonal_top.push((tile_rank as usize, tile_file as usize));
                }
                if tile_rank > rank {
                    diagonal_bottom.push((tile_rank as usize, tile_file as usize));
                }
            }
        }

        diagonal_top.reverse();
        for tile in diagonal_top {
            let query = self.query_board(tile.0, tile.1).0;
            if query.symbol == ChessPieces::None {
                moves.push(tile);
            } else {
                if query.white != white {
                    moves.push(tile);
                }
                break;
            }
        }

        for tile in diagonal_bottom {
            let query = self.query_board(tile.0, tile.1).0;
            if query.symbol == ChessPieces::None {
                moves.push(tile);
            } else {
                if query.white != white {
                    moves.push(tile);
                }
                break;
            }
        }

        moves
    }

    // Calculates all possible moves for any given piece, returns a vector of coordinates
    pub fn possible_moves(&self, origin_piece: Piece) -> Vec<(usize, usize)> {
        match origin_piece.symbol {
            ChessPieces::Pawn => {
                let mut moves = vec![];

                if origin_piece.white {
                    let query_front_tile =
                        self.query_board(origin_piece.rank - 1, origin_piece.file).0;
                    if query_front_tile.symbol == ChessPieces::None {
                        moves.push((origin_piece.rank - 1, origin_piece.file));
                        if origin_piece.rank == 6 {
                            let query_far_front_tile =
                                self.query_board(origin_piece.rank - 2, origin_piece.file).0;
                            if query_far_front_tile.symbol == ChessPieces::None {
                                moves.push((origin_piece.rank - 2, origin_piece.file));
                            }
                        }
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
                    let query_front_tile =
                        self.query_board(origin_piece.rank + 1, origin_piece.file).0;
                    if query_front_tile.symbol == ChessPieces::None {
                        moves.push((origin_piece.rank + 1, origin_piece.file));
                        if origin_piece.rank == 1 {
                            let query_far_front_tile =
                                self.query_board(origin_piece.rank + 2, origin_piece.file).0;
                            if query_far_front_tile.symbol == ChessPieces::None {
                                moves.push((origin_piece.rank + 2, origin_piece.file));
                            }
                        }
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

                let mut left_moves = self.valid_positions_left(
                    origin_piece.rank,
                    origin_piece.file,
                    origin_piece.white,
                );
                let mut right_moves = self.valid_positions_right(
                    origin_piece.rank,
                    origin_piece.file,
                    origin_piece.white,
                );
                let mut up_moves = self.valid_positions_up(
                    origin_piece.rank,
                    origin_piece.file,
                    origin_piece.white,
                );
                let mut down_moves = self.valid_positions_down(
                    origin_piece.rank,
                    origin_piece.file,
                    origin_piece.white,
                );

                moves.append(&mut left_moves);
                moves.append(&mut right_moves);
                moves.append(&mut up_moves);
                moves.append(&mut down_moves);

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

                let mut moves_positive_diag = self.valid_positions_diagonal_positive(
                    origin_piece.rank,
                    origin_piece.file,
                    origin_piece.white,
                );
                let mut moves_negative_diag = self.piece_moves_diagonal_negative(
                    origin_piece.rank,
                    origin_piece.file,
                    origin_piece.white,
                );

                moves.append(&mut moves_positive_diag);
                moves.append(&mut moves_negative_diag);

                moves
            }
            ChessPieces::Queen => {
                let mut moves = vec![];

                let mut left_moves = self.valid_positions_left(
                    origin_piece.rank,
                    origin_piece.file,
                    origin_piece.white,
                );
                let mut right_moves = self.valid_positions_right(
                    origin_piece.rank,
                    origin_piece.file,
                    origin_piece.white,
                );
                let mut up_moves = self.valid_positions_up(
                    origin_piece.rank,
                    origin_piece.file,
                    origin_piece.white,
                );
                let mut down_moves = self.valid_positions_down(
                    origin_piece.rank,
                    origin_piece.file,
                    origin_piece.white,
                );

                moves.append(&mut left_moves);
                moves.append(&mut right_moves);
                moves.append(&mut up_moves);
                moves.append(&mut down_moves);

                let mut moves_positive_diag = self.valid_positions_diagonal_positive(
                    origin_piece.rank,
                    origin_piece.file,
                    origin_piece.white,
                );
                let mut moves_negative_diag = self.piece_moves_diagonal_negative(
                    origin_piece.rank,
                    origin_piece.file,
                    origin_piece.white,
                );

                moves.append(&mut moves_positive_diag);
                moves.append(&mut moves_negative_diag);

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

    // Filters down the possible moves so that they don't include your own pieces
    pub fn filter_possible_moves(&self, origin_piece: Piece) -> Vec<(usize, usize)> {
        let mut filtered_moves = vec![];
        let possible_moves = self.possible_moves(origin_piece);
        for tile in possible_moves {
            let piece = self.query_board(tile.0, tile.1).0;
            if piece.symbol == ChessPieces::None || piece.white != origin_piece.white {
                filtered_moves.push(tile);
            }
        }
        filtered_moves
    }

    // Moves a given piece from its original location to (new_rank, new_file), returns whether or not the piece captured a king
    pub fn move_piece(&mut self, piece: Piece, new_rank: usize, new_file: usize) -> bool {
        let possible_moves = self.filter_possible_moves(piece);
        if possible_moves.contains(&(new_rank, new_file)) {
            let piece_at_position = self.query_board(new_rank, new_file).0;
            let old_rank = piece.rank;
            let old_file = piece.file;
            let moved_piece = Piece {
                symbol: piece.symbol,
                rank: new_rank,
                file: new_file,
                white: piece.white,
            };
            self.pieces[new_rank][new_file] = moved_piece;
            self.pieces[old_rank][old_file] = Piece {
                symbol: ChessPieces::None,
                rank: old_rank,
                file: old_file,
                white: true,
            };
            self.white_move = !self.white_move;
            return piece_at_position.symbol == ChessPieces::King;
        }
        self.moving = false;
        return false;
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

    // Returns the text symbol for the piece, including ANSI sequences
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
    // Converts a FEN character into a useable piece type
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

    // Returns the ASCII character for each piece
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
