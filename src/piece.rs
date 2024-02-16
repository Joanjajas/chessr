#[derive(Debug, Copy, Clone)]
pub enum Color {
    White = 0,
    Black = 1,
}

#[derive(Debug, Copy, Clone)]
pub enum PieceKind {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

#[derive(Debug, Copy, Clone)]
pub enum Piece {
    WhitePawn = 0b0001,
    WhiteKnight = 0b0010,
    WhiteBishop = 0b0011,
    WhiteRook = 0b0100,
    WhiteQueen = 0b0101,
    WhiteKing = 0b0110,
    BlackPawn = 0b1001,
    BlackKnight = 0b1010,
    BlackBishop = 0b1011,
    BlackRook = 0b1100,
    BlackQueen = 0b1101,
    BlackKing = 0b1110,
}

impl Piece {
    pub fn from_fen_char(c: char) -> Piece {
        match c {
            'P' => Piece::WhitePawn,
            'N' => Piece::WhiteKnight,
            'B' => Piece::WhiteBishop,
            'R' => Piece::WhiteRook,
            'Q' => Piece::WhiteQueen,
            'K' => Piece::WhiteKing,
            'p' => Piece::BlackPawn,
            'n' => Piece::BlackKnight,
            'b' => Piece::BlackBishop,
            'r' => Piece::BlackRook,
            'q' => Piece::BlackQueen,
            'k' => Piece::BlackKing,
            _ => panic!("Invalid FEN piece character: {}", c),
        }
    }

    pub fn color(&self) -> Color {
        match (*self as u8 & 0b1000) >> 3 {
            0 => Color::White,
            1 => Color::Black,
            _ => panic!("Invalid piece color"),
        }
    }

    pub fn kind(&self) -> PieceKind {
        match *self as u8 & 0b0111 {
            1 => PieceKind::Pawn,
            2 => PieceKind::Knight,
            3 => PieceKind::Bishop,
            4 => PieceKind::Rook,
            5 => PieceKind::Queen,
            6 => PieceKind::King,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let c = match *self {
            Piece::WhitePawn => '♟',
            Piece::WhiteKnight => '♞',
            Piece::WhiteBishop => '♝',
            Piece::WhiteRook => '♜',
            Piece::WhiteQueen => '♛',
            Piece::WhiteKing => '♚',
            Piece::BlackPawn => '♙',
            Piece::BlackKnight => '♘',
            Piece::BlackBishop => '♗',
            Piece::BlackRook => '♖',
            Piece::BlackQueen => '♕',
            Piece::BlackKing => '♔',
        };

        write!(f, "{}", c)
    }
}
