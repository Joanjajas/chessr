use crate::board::BitBoard;
use crate::board::Board;
use crate::castle::CastleRights;
use crate::color::Color;
use crate::consts::*;
use crate::piece::{Piece, PieceKind};
use crate::square::Square;

#[derive(Debug)]
pub enum FenParseError {
    Blocks,
    Rank,
    ConsecutiveDigits,
    RankSquares(usize),
    PawnRank(usize),
    MissingKing(Color),
    ActiveColor(String),
    CastleRight(char),
    EnPassantFile(char),
    EnPassantRank(char),
    EnPassantSquare(String),
    HalfmoveClock(std::num::ParseIntError),
    FullmoveNumber(std::num::ParseIntError),
}

impl std::fmt::Display for FenParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FenParseError::Blocks => writeln!(f, "Invalid number of blocks"),
            FenParseError::Rank => writeln!(f, "Invalid number of ranks"),
            FenParseError::ConsecutiveDigits => writeln!(f, "Consecutive digits in FEN string"),
            FenParseError::RankSquares(rank) => {
                writeln!(f, "Invalid number of squares in rank: {rank}")
            }
            FenParseError::PawnRank(rank) => writeln!(f, "Invalid pawn placement in rank {rank}"),
            FenParseError::MissingKing(color) => writeln!(f, "{color} king missing"),
            FenParseError::ActiveColor(color) => writeln!(f, "Invalid active color: {color}"),
            FenParseError::CastleRight(right) => writeln!(f, "Invalid castle right char: {right}"),
            FenParseError::EnPassantFile(file) => writeln!(f, "Invalid en passant file: {file}"),
            FenParseError::EnPassantRank(rank) => writeln!(f, "Invalid en passant rank: {rank}"),
            FenParseError::EnPassantSquare(square) => {
                writeln!(f, "Invalid en passant square: {square}")
            }
            FenParseError::HalfmoveClock(err) => {
                writeln!(f, "Invalid halfmove clock value: {err}")
            }
            FenParseError::FullmoveNumber(err) => {
                writeln!(f, "Invalid fullmove number value: {err}")
            }
        }
    }
}

impl std::error::Error for FenParseError {}

pub fn parse_fen(fen_str: &str) -> Result<Board, FenParseError> {
    let blocks: Vec<&str> = fen_str.split_whitespace().collect();

    // FEN string must have at least 4 blocks plus 2 optional blocks
    if blocks.len() < 4 || blocks.len() > 6 {
        return Err(FenParseError::Blocks);
    }

    let rows: Vec<&str> = blocks[0].split('/').collect();

    // FEN string must have 8 rows
    if rows.len() != 8 {
        return Err(FenParseError::Rank);
    }

    let mut pieces_order = [None; 64];
    let mut both_players_pieces = [BitBoard(0); PIECE_TYPE_COUNT];
    let mut players_pieces = [BitBoard(0); PLAYERS_COUNT];

    // set pieces on the board
    for (i, row) in rows.iter().enumerate() {
        let mut col = 0;
        let mut last_was_digit = false;
        let mut row_sum = 0;

        for c in row.chars() {
            if c.is_ascii_digit() {
                if last_was_digit {
                    return Err(FenParseError::ConsecutiveDigits);
                }

                col += c.to_digit(10).unwrap() as usize;
                row_sum += c.to_digit(10).unwrap() as usize;
                last_was_digit = true;
            } else {
                let square = i * 8 + col;
                let piece = Piece::from_fen_char(c);

                // assign piece to the board
                pieces_order[square] = Some(piece);
                both_players_pieces[piece.kind() as usize] |= Square(square as u8).to_bb();
                players_pieces[piece.color() as usize] |= Square(square as u8).to_bb();

                col += 1;
                last_was_digit = false;
                row_sum += 1;
            }
        }

        // each row should have exactly 8 squares
        if row_sum != 8 {
            return Err(FenParseError::RankSquares(row_sum));
        }
    }

    // the board should'n have a pawn on the first rank
    if RANK_1 & both_players_pieces[PieceKind::Pawn as usize].0 != 0 {
        return Err(FenParseError::PawnRank(1));
    }

    // the board should'n have a pawn on the last rank
    if RANK_8 & both_players_pieces[PieceKind::Pawn as usize].0 != 0 {
        return Err(FenParseError::PawnRank(8));
    }

    // white king is missing
    if both_players_pieces[PieceKind::King as usize].0 & players_pieces[Color::White as usize].0
        == 0
    {
        return Err(FenParseError::MissingKing(Color::White));
    }

    // black king is missing
    if both_players_pieces[PieceKind::King as usize].0 & players_pieces[Color::Black as usize].0
        == 0
    {
        return Err(FenParseError::MissingKing(Color::Black));
    }

    let active_color = match blocks[1] {
        "w" => Color::White,
        "b" => Color::Black,
        color => return Err(FenParseError::ActiveColor(color.to_string())),
    };

    let mut castle_rights = CastleRights(0);
    match blocks[2] {
        "-" => (),
        rights => {
            for c in rights.chars() {
                castle_rights.0 |= match c {
                    'K' => WHITE_KINGSIDE_CASTLE,
                    'Q' => WHITE_QUEENSIDE_CASTLE,
                    'k' => BLACK_KINGSIDE_CASTLE,
                    'q' => BLACK_QUEENSIDE_CASTLE,
                    _ => return Err(FenParseError::CastleRight(c)),
                }
            }
        }
    }

    let en_passant_target = match blocks[3] {
        "-" => None,
        square => {
            let mut ep_square = Square(0);
            for (i, char) in square.chars().enumerate() {
                if i == 0 {
                    match char {
                        'a' => ep_square = Square(0),
                        'b' => ep_square = Square(1),
                        'c' => ep_square = Square(2),
                        'd' => ep_square = Square(3),
                        'e' => ep_square = Square(4),
                        'f' => ep_square = Square(5),
                        'g' => ep_square = Square(6),
                        'h' => ep_square = Square(7),
                        _ => return Err(FenParseError::EnPassantFile(char)),
                    }
                }

                if i == 1 {
                    match char {
                        '3' if active_color == Color::Black => ep_square += Square(16),
                        '6' if active_color == Color::White => ep_square += Square(40),
                        _ => return Err(FenParseError::EnPassantRank(char)),
                    }
                }

                if i > 1 {
                    return Err(FenParseError::EnPassantSquare(square.to_string()));
                }
            }

            if ep_square.0 == 0 {
                None
            } else {
                Some(ep_square)
            }
        }
    };

    let halfmove_clock = match blocks.get(4).unwrap_or(&"0").parse() {
        Ok(value) => value,
        Err(err) => return Err(FenParseError::HalfmoveClock(err)),
    };

    let fullmove_number = match blocks.get(5).unwrap_or(&"1").parse() {
        Ok(value) => value,
        Err(err) => return Err(FenParseError::FullmoveNumber(err)),
    };

    Ok(Board {
        pieces_order,
        players_pieces,
        both_players_pieces,
        active_color,
        castle_rights,
        en_passant_target,
        halfmove_clock,
        fullmove_number,
    })
}
