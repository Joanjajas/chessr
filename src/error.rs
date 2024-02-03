/// Represents errors that can occur when parsing a FEN string.
#[derive(Debug)]
pub enum FenParseError {
    FenString,
    PiecePositions,
    ActiveColor,
    CastleRights,
    EnPassant,
    HalfmoveClock,
    FullmoveNumber,
}

impl std::error::Error for FenParseError {}

impl std::fmt::Display for FenParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FenParseError::FenString => write!(f, "Invalid FEN string"),
            FenParseError::PiecePositions => write!(f, "Invalid piece positions"),
            FenParseError::ActiveColor => write!(f, "Invalid active color"),
            FenParseError::CastleRights => write!(f, "Invalid castle rights"),
            FenParseError::EnPassant => write!(f, "Invalid en passant"),
            FenParseError::HalfmoveClock => write!(f, "Invalid halfmove clock"),
            FenParseError::FullmoveNumber => write!(f, "Invalid fullmove number"),
        }
    }
}
