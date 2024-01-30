use crate::castle::{CastleKind, CastleRights};
use crate::color::Color;
use crate::constants::*;
use crate::error::FenParseError;
use crate::fen;
use crate::piece::Piece;
use crate::r#move::Move;

#[derive(Debug, Clone)]
pub struct Board {
    pub pieces: [[Option<Piece>; 8]; 8],
    pub active_color: Color,
    pub castle_rights: Vec<CastleRights>,
    pub en_passant: Option<(usize, usize)>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

impl Board {
    /// Creates a new board with the starting position.
    pub fn new() -> Board {
        fen::fen_to_board(FEN_STARTING_POSITION).unwrap()
    }

    /// Creates a new board from a FEN string.
    pub fn from_fen(fen_str: &str) -> Result<Board, FenParseError> {
        fen::fen_to_board(fen_str)
    }

    /// Returns a FEN string representation of the the board.
    pub fn fen(&self) -> String {
        fen::board_to_fen(self)
    }

    // Returns the piece at the given square, if any.
    pub fn get_piece(&self, square: (usize, usize)) -> Option<Piece> {
        if (0..=7).contains(&square.0) && (0..=7).contains(&square.1) {
            return self.pieces[square.0][square.1];
        }

        None
    }

    // Sets the piece at the given square.
    pub fn set_piece(&mut self, square: (usize, usize), piece: Option<Piece>) {
        if (0..=7).contains(&square.0) && (0..=7).contains(&square.1) {
            self.pieces[square.0][square.1] = piece;
        }
    }

    /// Makes a move on the board given its algebraic notation.
    /// If the move notation is invalid or the move is not legal, no move is
    /// made.
    /// Returns the move that was made.
    pub fn make_move_algebraic(&mut self, r#move: &str) -> Option<Move> {
        let r#move = Move::from_algebraic(r#move, self);
        let initial_state = self.clone();
        let initial_check = initial_state.check();

        if let Some(ref r#move) = r#move {
            // handle en passant capture
            if r#move.en_passant_capture {
                let en_passant_square = self.en_passant?;

                // calculate the square of the pawn that was captured
                let en_passant_capture_square = match self.active_color {
                    Color::White => (en_passant_square.0 + 1, en_passant_square.1),
                    Color::Black => (en_passant_square.0 - 1, en_passant_square.1),
                };

                self.set_piece(en_passant_capture_square, None);
            }

            // handle castling
            if let Some(ref castle) = r#move.castle {
                match castle {
                    CastleKind::Kingside => self.castle_kingside(self.active_color),
                    CastleKind::Queenside => self.castle_queenside(self.active_color),
                }
            }

            // update the board state
            self.update_castle_rights(r#move);
            self.active_color = self.active_color.invert();
            self.en_passant = r#move.en_passant;
            self.halfmove_clock += 1;

            // fullmove number (increases every turn)
            self.fullmove_number += match self.active_color {
                Color::White => 1,
                Color::Black => 0,
            };

            let src_square = r#move.src_square?;
            let dst_square = r#move.dst_square?;
            let src_square_piece = self.get_piece(src_square);
            let dst_square_piece = self.get_piece(dst_square);

            // reset  halfmove clock if a pawn is moved or a piece is captured
            if src_square_piece == Some(Piece::Pawn(self.active_color.invert()))
                || dst_square_piece.is_some()
                || r#move.en_passant_capture
            {
                self.halfmove_clock = 0;
            }

            // handle promotion
            if let Some(promotion_piece) = r#move.promotion {
                self.set_piece(dst_square, Some(promotion_piece));
            } else {
                self.set_piece(dst_square, src_square_piece);
            }

            if Piece::King(self.active_color) != src_square_piece? {
                self.set_piece(src_square, None);
            }
        }

        let last_check = self.check();
        if initial_check.is_some() && last_check.is_some() && (initial_check == last_check)
            || last_check.is_some_and(|c| c == self.active_color)
        {
            self.active_color = initial_state.active_color;
            self.castle_rights = initial_state.castle_rights;
            self.en_passant = initial_state.en_passant;
            self.halfmove_clock = initial_state.halfmove_clock;
            self.fullmove_number = initial_state.fullmove_number;
            self.pieces = initial_state.pieces;
        }

        r#move
    }

    // Returns the color of the king that is in check, if any.
    fn check(&self) -> Option<Color> {
        for &color in [Color::White, Color::Black].iter() {
            let mut king_square: (i8, i8) = (0, 0);
            for (i, row) in self.pieces.iter().enumerate() {
                for (j, piece) in row.iter().enumerate() {
                    if piece == &Some(Piece::King(color.invert())) {
                        king_square = (i as i8, j as i8);
                        break;
                    }
                }
            }

            let piece_directions = vec![
                (Piece::Pawn(color), PAWN_CAPTURE_DIRECTIONS.to_vec()),
                (Piece::Knight(color), KNIGHT_DIRECTIONS.to_vec()),
                (Piece::Bishop(color), BISHOP_DIRECTIONS.to_vec()),
                (Piece::Rook(color), ROOK_DIRECTIONS.to_vec()),
                (Piece::Queen(color), QUEEN_DIRECTIONS.to_vec()),
                (Piece::King(color), KING_DIRECTIONS.to_vec()),
            ];

            for (piece, directions) in piece_directions.iter() {
                for direction in directions.iter() {
                    let mut king_square =
                        (king_square.0 + direction.0, king_square.1 + direction.1);

                    while (0..=7).contains(&king_square.0) && (0..=7).contains(&king_square.1) {
                        let src_square_piece =
                            self.get_piece((king_square.0 as usize, king_square.1 as usize));

                        if src_square_piece.is_some_and(|p| &p != piece) {
                            break;
                        }

                        if src_square_piece.is_none() {
                            king_square.0 += direction.0;
                            king_square.1 += direction.1;

                            match piece {
                                Piece::Queen(_) => continue,
                                Piece::Rook(_) => continue,
                                Piece::Bishop(_) => continue,
                                Piece::Knight(_) => break,
                                Piece::King(_) => break,
                                Piece::Pawn(_) => break,
                            }
                        }

                        return Some(color);
                    }
                }
            }
        }

        None
    }

    fn castle_kingside(&mut self, color: Color) {
        let row = match color {
            Color::White => 7,
            Color::Black => 0,
        };

        let king_square = (row, 4);
        let rook_square = (row, 7);

        self.set_piece(king_square, None);
        self.set_piece(rook_square, None);

        let king = Piece::King(color);
        let rook = Piece::Rook(color);

        self.set_piece((row, 6), Some(king));
        self.set_piece((row, 5), Some(rook));
    }

    fn castle_queenside(&mut self, color: Color) {
        let row = match color {
            Color::White => 7,
            Color::Black => 0,
        };

        let king_square = (row, 4);
        let rook_square = (row, 0);

        self.set_piece(king_square, None);
        self.set_piece(rook_square, None);

        let king = Piece::King(color);
        let rook = Piece::Rook(color);

        self.set_piece((row, 2), Some(king));
        self.set_piece((row, 3), Some(rook));
    }

    fn update_castle_rights(&mut self, r#move: &Move) {
        if r#move.castle.is_some() {
            match self.active_color {
                Color::White => self.castle_rights.retain(|x| {
                    x != &CastleRights::WhiteKingside && x != &CastleRights::WhiteQueenside
                }),
                Color::Black => self.castle_rights.retain(|x| {
                    x != &CastleRights::BlackKingside && x != &CastleRights::BlackQueenside
                }),
            }
        }

        if let Some(src_square) = r#move.src_square {
            let src_square_piece = self.get_piece(src_square);

            match src_square_piece {
                Some(Piece::King(Color::White)) => {
                    self.castle_rights.retain(|x| {
                        x != &CastleRights::WhiteKingside && x != &CastleRights::WhiteQueenside
                    });
                }

                Some(Piece::King(Color::Black)) => {
                    self.castle_rights.retain(|x| {
                        x != &CastleRights::BlackKingside && x != &CastleRights::BlackQueenside
                    });
                }

                Some(Piece::Rook(Color::White)) => {
                    if src_square == (7, 7) {
                        self.castle_rights
                            .retain(|x| x != &CastleRights::WhiteKingside);
                    } else if src_square == (7, 0) {
                        self.castle_rights
                            .retain(|x| x != &CastleRights::WhiteQueenside);
                    }
                }

                Some(Piece::Rook(Color::Black)) => {
                    if src_square == (0, 7) {
                        self.castle_rights
                            .retain(|x| x != &CastleRights::BlackKingside);
                    } else if src_square == (0, 0) {
                        self.castle_rights
                            .retain(|x| x != &CastleRights::BlackQueenside);
                    }
                }

                _ => {}
            }
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

/// Tries to convert an algebraic notation string into a tuple of coordinates
pub fn algebraic_to_coordinates(algebraic: &str) -> Option<(usize, usize)> {
    let mut chars = algebraic.chars();
    let column_char = chars.next()?;
    let row_char = chars.next()?;

    if !('a'..='h').contains(&column_char) || !('1'..='8').contains(&row_char) {
        return None;
    }

    // 7 - () because the board is zero-indexed and the rows are reversed
    let row = 7 - (row_char as usize - 49);
    let column = column_char as usize - 97;

    Some((row, column))
}

/// Tries to convert a tuple of coordinates into an algebraic notation string
pub fn coordinates_to_algebraic(coordinates: (usize, usize)) -> Option<String> {
    let (row, column) = coordinates;

    if !(0..=7).contains(&row) || !(0..=7).contains(&column) {
        return None;
    }

    let row_char = 8 - row;
    let column_char = column as u8 + 97;

    let algebraic = format!("{}{}", column_char as char, row_char);
    Some(algebraic)
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();

        for row in self.pieces.iter() {
            for piece in row.iter() {
                match piece {
                    Some(Piece::Pawn(Color::Black)) => board.push('♙'),
                    Some(Piece::Knight(Color::Black)) => board.push('♘'),
                    Some(Piece::Bishop(Color::Black)) => board.push('♗'),
                    Some(Piece::Rook(Color::Black)) => board.push('♖'),
                    Some(Piece::Queen(Color::Black)) => board.push('♕'),
                    Some(Piece::King(Color::Black)) => board.push('♔'),
                    Some(Piece::Pawn(Color::White)) => board.push('♟'),
                    Some(Piece::Knight(Color::White)) => board.push('♞'),
                    Some(Piece::Bishop(Color::White)) => board.push('♝'),
                    Some(Piece::Rook(Color::White)) => board.push('♜'),
                    Some(Piece::Queen(Color::White)) => board.push('♛'),
                    Some(Piece::King(Color::White)) => board.push('♚'),
                    None => board.push(' '),
                }
            }
            board.push('\n');
        }

        write!(f, "{}", board)
    }
}
