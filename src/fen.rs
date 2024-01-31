use crate::board::Board;
use crate::castle::CastleRights;
use crate::color::Color;
use crate::conversion::{algebraic_to_coordinates, coordinates_to_algebraic};
use crate::error::FenParseError;
use crate::piece::Piece;

/// Creates a new board from a FEN string
pub fn fen_to_board(fen_string: &str) -> Result<Board, FenParseError> {
    let mut pieces = [[None; 8]; 8];
    let fen_blocks: Vec<&str> = fen_string.split_whitespace().collect();

    // the FEN string should have at least 4 blocks and not more than 6
    if fen_blocks.len() < 4 || fen_blocks.len() > 6 {
        return Err(FenParseError::FenString);
    }

    let piece_placement = fen_blocks
        .first()
        .ok_or(FenParseError::FenString)?
        .split('/');

    // set the pieces for each row
    for (i, row) in piece_placement.enumerate() {
        let mut col = 0;
        let mut row_count = 0;

        for c in row.chars() {
            if row_count > 7 {
                return Err(FenParseError::PiecePositions);
            }

            if c.is_ascii_digit() {
                let digit = c.to_digit(10).ok_or(FenParseError::PiecePositions)? as usize;
                col += digit;
                row_count += digit;
            }

            if c.is_ascii_alphabetic() {
                let piece = Piece::from_fen_char(c).ok_or(FenParseError::PiecePositions)?;
                pieces[i][col] = Some(piece);
                col += 1;
                row_count += 1;
            }
        }

        if row_count != 8 {
            return Err(FenParseError::PiecePositions);
        }
    }

    let active_color = match *fen_blocks.get(1).ok_or(FenParseError::FenString)? {
        "w" => Color::White,
        "b" => Color::Black,
        _ => return Err(FenParseError::ActiveColor),
    };

    let mut castle_rights = Vec::new();
    for c in fen_blocks.get(2).ok_or(FenParseError::FenString)?.chars() {
        match c {
            '-' => continue,
            _ => castle_rights
                .push(CastleRights::from_fen_char(c).ok_or(FenParseError::CastleRights)?),
        }
    }

    let en_passant = match *fen_blocks.get(3).ok_or(FenParseError::FenString)? {
        "-" => None,
        s => Some(algebraic_to_coordinates(s).ok_or(FenParseError::EnPassant)?),
    };

    // optional fields
    let halfmove_clock = match fen_blocks.get(4) {
        Some(s) => s.parse::<u32>().map_err(|_| FenParseError::HalfmoveClock)?,
        None => 0,
    };

    let fullmove_number = match fen_blocks.get(5) {
        Some(s) => s
            .parse::<u32>()
            .map_err(|_| FenParseError::FullmoveNumber)?,
        None => 1,
    };

    Ok(Board {
        pieces,
        active_color,
        castle_rights,
        en_passant,
        halfmove_clock,
        fullmove_number,
    })
}

pub fn board_to_fen(board: &Board) -> String {
    let mut fen = String::new();

    // piece placement
    for row in board.pieces.iter() {
        let mut empty_squares = 0;

        for piece in row.iter() {
            match piece {
                Some(p) => {
                    if empty_squares > 0 {
                        fen.push_str(&empty_squares.to_string());
                        empty_squares = 0;
                    }

                    fen.push_str(&p.fen().to_string());
                }
                None => empty_squares += 1,
            }
        }

        if empty_squares > 0 {
            fen.push_str(&empty_squares.to_string());
        }

        fen.push('/');
    }

    fen.pop(); // remove the last slash
    fen.push(' ');

    // active color
    fen.push_str(&board.active_color.fen().to_string());
    fen.push(' ');

    // castle rights
    if board.castle_rights.is_empty() {
        fen.push('-');
    } else {
        for right in board.castle_rights.iter() {
            fen.push_str(&right.fen().to_string());
        }
    }

    fen.push(' ');

    // en passant
    match board.en_passant {
        Some((row, column)) => {
            let algebraic = coordinates_to_algebraic((row, column)).unwrap_or("-".to_string());
            fen.push_str(&algebraic);
        }
        None => fen.push('-'),
    }

    fen.push(' ');

    // halfmove clock
    fen.push_str(&board.halfmove_clock.to_string());
    fen.push(' ');

    // fullmove number
    fen.push_str(&board.fullmove_number.to_string());

    fen
}
