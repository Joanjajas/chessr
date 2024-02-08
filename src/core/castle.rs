use crate::core::Color;

/// Represents a castle kind.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CastleKind {
    Kingside,
    Queenside,
}

impl CastleKind {
    /// Tries to create a castle kind from the given SAN string.
    pub fn from_san_str(str: &str) -> Option<CastleKind> {
        match str {
            "O-O" | "0-0" | "o-o" => Some(CastleKind::Kingside),
            "O-O-O" | "0-0-0" | "o-o-o" => Some(CastleKind::Queenside),
            _ => None,
        }
    }

    /// Tries to create a castle kind from the given UCI notation string.
    pub fn from_uci_str(uci: &str) -> Option<CastleKind> {
        match uci {
            "e1g1" | "e8g8" | "e1-g1" | "e8-g8" => Some(CastleKind::Kingside),
            "e1c1" | "e8c8" | "e1-c1" | "e8-c8" => Some(CastleKind::Queenside),
            _ => None,
        }
    }

    /// Returns a SAN string of the castle kind.
    pub fn to_san_str(&self) -> String {
        match self {
            CastleKind::Kingside => "O-O".to_string(),
            CastleKind::Queenside => "O-O-O".to_string(),
        }
    }

    /// Returns an UCI notation string of the castle kind.
    pub fn to_uci_str(&self, color: &Color) -> String {
        match self {
            CastleKind::Kingside => match color {
                Color::White => "e1g1".to_string(),
                Color::Black => "e8g8".to_string(),
            },
            CastleKind::Queenside => match color {
                Color::White => "e1c1".to_string(),
                Color::Black => "e8c8".to_string(),
            },
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
    pub fn to_fen_char(&self) -> char {
        match self {
            CastleRights::WhiteKingside => 'K',
            CastleRights::WhiteQueenside => 'Q',
            CastleRights::BlackKingside => 'k',
            CastleRights::BlackQueenside => 'q',
        }
    }
}
