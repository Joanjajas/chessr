use crate::color::Color;

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
    /// Returns the color of the piece.
    pub fn color(&self) -> &Color {
        match self {
            Piece::Pawn(color) => color,
            Piece::Knight(color) => color,
            Piece::Bishop(color) => color,
            Piece::Rook(color) => color,
            Piece::Queen(color) => color,
            Piece::King(color) => color,
        }
    }

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

    /// Returns a FEN representation of the piece.
    pub fn fen(&self) -> char {
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

    /// Tries to create a piece from an algebraic notation character.
    pub fn from_algebraic_char(c: char, color: Color) -> Option<Piece> {
        match c.to_ascii_lowercase() {
            'p' => Some(Piece::Pawn(color)),
            'n' => Some(Piece::Knight(color)),
            'b' => Some(Piece::Bishop(color)),
            'r' => Some(Piece::Rook(color)),
            'q' => Some(Piece::Queen(color)),
            'k' => Some(Piece::King(color)),
            _ => None,
        }
    }
}
