use crate::constants::{
    BISHOP_DIRECTIONS, KING_DIRECTIONS, KNIGHT_DIRECTIONS, PAWN_DIRECTIONS, QUEEN_DIRECTIONS,
    ROOK_DIRECTIONS,
};
use crate::core::Color;

/// Represents a chess piece.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Piece {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl Piece {
    /// Tries to create a piece from a FEN character.
    pub fn from_fen_char(c: char) -> Option<Piece> {
        match c {
            'p' => Some(Piece::Pawn(Color::Black)),
            'n' => Some(Piece::Knight(Color::Black)),
            'b' => Some(Piece::Bishop(Color::Black)),
            'r' => Some(Piece::Rook(Color::Black)),
            'q' => Some(Piece::Queen(Color::Black)),
            'k' => Some(Piece::King(Color::Black)),
            'P' => Some(Piece::Pawn(Color::White)),
            'N' => Some(Piece::Knight(Color::White)),
            'B' => Some(Piece::Bishop(Color::White)),
            'R' => Some(Piece::Rook(Color::White)),
            'Q' => Some(Piece::Queen(Color::White)),
            'K' => Some(Piece::King(Color::White)),
            _ => None,
        }
    }

    /// Tries to create a piece from a SAN character.
    pub fn from_san_char(c: char, color: Color) -> Option<Piece> {
        match c {
            'P' => Some(Piece::Pawn(color)),
            'N' => Some(Piece::Knight(color)),
            'B' => Some(Piece::Bishop(color)),
            'R' => Some(Piece::Rook(color)),
            'Q' => Some(Piece::Queen(color)),
            'K' => Some(Piece::King(color)),
            _ => None,
        }
    }

    /// Tries to create a piece from a UCI notation character.
    pub fn from_uci_char(c: char, color: Color) -> Option<Piece> {
        match c {
            'p' => Some(Piece::Pawn(color)),
            'n' => Some(Piece::Knight(color)),
            'b' => Some(Piece::Bishop(color)),
            'r' => Some(Piece::Rook(color)),
            'q' => Some(Piece::Queen(color)),
            'k' => Some(Piece::King(color)),
            _ => None,
        }
    }

    /// Returns a FEN representation of the piece.
    pub fn to_fen_char(&self) -> char {
        match self {
            Piece::Pawn(Color::Black) => 'p',
            Piece::Knight(Color::Black) => 'n',
            Piece::Bishop(Color::Black) => 'b',
            Piece::Rook(Color::Black) => 'r',
            Piece::Queen(Color::Black) => 'q',
            Piece::King(Color::Black) => 'k',
            Piece::Pawn(Color::White) => 'P',
            Piece::Knight(Color::White) => 'N',
            Piece::Bishop(Color::White) => 'B',
            Piece::Rook(Color::White) => 'R',
            Piece::Queen(Color::White) => 'Q',
            Piece::King(Color::White) => 'K',
        }
    }

    /// Returns a SAN representation of the piece.
    pub fn to_san_char(&self) -> char {
        match self {
            Piece::Pawn(_) => 'P',
            Piece::Knight(_) => 'N',
            Piece::Bishop(_) => 'B',
            Piece::Rook(_) => 'R',
            Piece::Queen(_) => 'Q',
            Piece::King(_) => 'K',
        }
    }

    /// Returns an UCI notation string representation of the piece.
    pub fn to_uci_char(&self) -> char {
        match self {
            Piece::Pawn(_) => 'p',
            Piece::Knight(_) => 'n',
            Piece::Bishop(_) => 'b',
            Piece::Rook(_) => 'r',
            Piece::Queen(_) => 'q',
            Piece::King(_) => 'k',
        }
    }

    /// Returns the color of the piece.
    pub fn color(&self) -> Color {
        match self {
            Piece::Pawn(color) => *color,
            Piece::Knight(color) => *color,
            Piece::Bishop(color) => *color,
            Piece::Rook(color) => *color,
            Piece::Queen(color) => *color,
            Piece::King(color) => *color,
        }
    }

    /// Returns the directions in which the piece can move in.
    pub fn directions(&self) -> Vec<(i8, i8)> {
        match self {
            Piece::Pawn(Color::Black) => PAWN_DIRECTIONS.to_vec(),
            Piece::Pawn(Color::White) => PAWN_DIRECTIONS.iter().map(|(x, y)| (-x, -y)).collect(),
            Piece::Knight(_) => KNIGHT_DIRECTIONS.to_vec(),
            Piece::Bishop(_) => BISHOP_DIRECTIONS.to_vec(),
            Piece::Rook(_) => ROOK_DIRECTIONS.to_vec(),
            Piece::Queen(_) => QUEEN_DIRECTIONS.to_vec(),
            Piece::King(_) => KING_DIRECTIONS.to_vec(),
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match self {
            Piece::Pawn(Color::White) => '♟',
            Piece::Knight(Color::White) => '♞',
            Piece::Bishop(Color::White) => '♝',
            Piece::Rook(Color::White) => '♜',
            Piece::Queen(Color::White) => '♛',
            Piece::King(Color::White) => '♚',
            Piece::Pawn(Color::Black) => '♙',
            Piece::Knight(Color::Black) => '♘',
            Piece::Bishop(Color::Black) => '♗',
            Piece::Rook(Color::Black) => '♖',
            Piece::Queen(Color::Black) => '♕',
            Piece::King(Color::Black) => '♔',
        };
        write!(f, "{}", c)
    }
}
