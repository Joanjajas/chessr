/// Represents a castle kind.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CastleKind {
    Kingside,
    Queenside,
}

impl CastleKind {
    /// Tries to create a castle kind from an algebraic notation.
    pub fn from_algebraic(str: &str) -> Option<CastleKind> {
        match str {
            "O-O" | "0-0" | "o-o" => Some(CastleKind::Kingside),
            "O-O-O" | "0-0-0" | "o-o-o" => Some(CastleKind::Queenside),
            _ => None,
        }
    }
}

/// Represents the castle rights of a player.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CastleRights {
    WhiteKingside,
    WhiteQueenside,
    BlackKingside,
    BlackQueenside,
}

impl CastleRights {
    /// Tries to create a castle right from a FEN character.
    pub fn from_fen_char(c: char) -> Option<CastleRights> {
        match c {
            'K' => Some(CastleRights::WhiteKingside),
            'Q' => Some(CastleRights::WhiteQueenside),
            'k' => Some(CastleRights::BlackKingside),
            'q' => Some(CastleRights::BlackQueenside),
            _ => None,
        }
    }

    /// Returns a FEN representation of the castle right.
    pub fn fen(&self) -> char {
        match self {
            CastleRights::WhiteKingside => 'K',
            CastleRights::WhiteQueenside => 'Q',
            CastleRights::BlackKingside => 'k',
            CastleRights::BlackQueenside => 'q',
        }
    }
}
