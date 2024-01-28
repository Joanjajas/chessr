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

    pub fn invert(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
