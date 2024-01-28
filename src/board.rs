use crate::castle::CastleRights;
use crate::color::Color;
use crate::error::FenParseError;
use crate::fen;
use crate::piece::Piece;
use crate::r#move::Move;

const FEN_STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug)]
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

    pub fn make_move_algebraic(&mut self, r#move: &str) -> Option<Move> {
        let r#move = Move::from_algebraic(r#move, self);

        if let Some(ref r#move) = r#move {
            // handle en passant
            if r#move.en_passant_capture {
                let en_passant_square = self.en_passant?;

                let en_passant_capture_square = match self.active_color {
                    Color::White => (en_passant_square.0 + 1, en_passant_square.1),
                    Color::Black => (en_passant_square.0 - 1, en_passant_square.1),
                };

                self.set_piece(en_passant_capture_square, None);
            }

            self.active_color = self.active_color.invert();
            let src_square = r#move.src_square?;
            let dst_square = r#move.dst_square?;

            // reset halfmove clock (if pawn moved or piece captured)
            if self.get_piece(dst_square).is_some()
                || self.get_piece(src_square) == Some(Piece::Pawn(self.active_color))
            {
                self.halfmove_clock = 0;
            } else {
                self.halfmove_clock += 1;
            }

            // handle promotion
            match r#move.promotion {
                Some(promotion) => {
                    self.set_piece(dst_square, Some(promotion));
                }
                None => {
                    self.set_piece(dst_square, self.get_piece(src_square));
                }
            }

            self.set_piece(src_square, None);
            self.en_passant = r#move.en_passant;

            if self.active_color == Color::White {
                self.fullmove_number += 1;
            }
        }

        r#move
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
    let row_char = chars.next()?;
    let column_char = chars.next()?;

    if !('a'..='h').contains(&row_char) || !('1'..='8').contains(&column_char) {
        return None;
    }

    // 7 - () because the board is zero-indexed and the rows are reversed
    let row = 7 - (column_char as usize - 49);

    // -1 because the board is zero-indexed
    let column = row_char as usize - 97;

    Some((row, column))
}

/// Tries to convert a tuple of coordinates into an algebraic notation string
pub fn coordinates_to_algebraic(coordinates: (usize, usize)) -> Option<String> {
    let (row, column) = coordinates;

    if !(0..=7).contains(&row) || !(0..=7).contains(&column) {
        return None;
    }

    // 7 - () because the board is zero-indexed and the rows are reversed
    let row_char = (7 - row) as u8 + 49;

    // + 1 because the board is zero-indexed
    let column_char = column as u8 + 1;

    let mut algebraic = String::new();
    algebraic.push(row_char as char);
    algebraic.push(column_char as char);

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
