/// Represents the color of a piece or a player.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Returns a FEN representation of the color.
    pub fn fen(self) -> char {
        match self {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }

    /// Inverts the color.
    /// White -> Black
    /// Black -> White
    pub fn invert(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
