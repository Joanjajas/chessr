use crate::board::Board;
use crate::castle::{CastleKind, CastleRights};
use crate::color::Color;
use crate::constants::*;
use crate::conversion::algebraic_to_coordinates;
use crate::piece::Piece;

use regex::Regex;

/// Struct representing a chess move.
#[derive(Debug, Clone)]
pub struct Move {
    /// Source square of the piece moving
    pub src_square: Option<(usize, usize)>,

    /// Destination square of the piece moving
    pub dst_square: Option<(usize, usize)>,

    /// En passant target square
    pub en_passant: Option<(usize, usize)>,

    /// Whether the move is an en passant capture
    pub en_passant_capture: bool,

    /// Castle type
    pub castle: Option<CastleKind>,

    /// Piece to promote.
    pub promotion: Option<Piece>,
}

impl Move {
    /// Returns a [Move] struct representation of the given algebraic notation for the given board.
    /// If the move is invalid or illega `None` will be returned
    pub fn from_algebraic(r#move: &str, board: &Board) -> Option<Move> {
        // castling
        let re = Regex::new(CASTLE_REGEX).expect("Invalid castle regex");

        if re.is_match(r#move) {
            let castle_type = CastleKind::from_algebraic(r#move)?;
            return castle(castle_type, board);
        };

        // pawn move
        let re = Regex::new(PAWN_MOVE_REGEX).expect("Invalid pawn move regex");

        if re.is_match(r#move) {
            let dst_square = algebraic_to_coordinates(r#move)?;
            return pawn_move(dst_square, board);
        }

        // piece move
        let re = Regex::new(PIECE_MOVE_REGEX).expect("Invalid piece move regex");

        if re.is_match(r#move) {
            let piece = Piece::from_algebraic_char(r#move.chars().next()?, board.active_color)?;
            let dst_square = algebraic_to_coordinates(&r#move[1..])?;

            return piece_move(piece, dst_square, None, None, board);
        }

        // piece disambiguation (row)
        let re = Regex::new(PIECE_MOVE_ROW_DISAMBIGUATION_REGEX)
            .expect("Invalid piece move row disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_algebraic_char(chars.next()?, board.active_color)?;
            let dst_square = algebraic_to_coordinates(&r#move[2..])?;
            let disambiguation_row = 7 - (chars.next()? as usize - 49);

            return piece_move(piece, dst_square, Some(disambiguation_row), None, board);
        }

        // piece disambiguation (column)
        let re = Regex::new(PIECE_MOVE_COLUMN_DISAMBIGUATION_REGEX)
            .expect("Invalid piece move column disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_algebraic_char(chars.next().unwrap(), board.active_color)?;
            let dst_square = algebraic_to_coordinates(&r#move[2..])?;
            let disambiguation_column = chars.next()? as usize - 97;

            return piece_move(piece, dst_square, None, Some(disambiguation_column), board);
        }

        // piece disambiguation (row and column)
        let re = Regex::new(PIECE_MOVE_ROW_AND_COLUMN_DISAMBIGUATION_REGEX)
            .expect("Invalid piece move row and column disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_algebraic_char(chars.next()?, board.active_color)?;
            let dst_square = algebraic_to_coordinates(&r#move[3..])?;
            let (disambiguation_row, disambiguation_column) =
                algebraic_to_coordinates(&r#move[1..3])?;

            return piece_move(
                piece,
                dst_square,
                Some(disambiguation_row),
                Some(disambiguation_column),
                board,
            );
        }

        // pawn capture
        let re = Regex::new(PAWN_CAPTURE_REGEX).expect("Invalid pawn capture regex");

        if re.is_match(r#move) {
            let dst_square = algebraic_to_coordinates(&r#move[2..])?;
            let disambiguation_column = r#move.chars().nth(0)? as usize - 97;

            return pawn_capture(dst_square, disambiguation_column, board);
        }

        // piece capture
        let re = Regex::new(PIECE_CAPTURE_REGEX).expect("Invalid piece capture regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_algebraic_char(chars.next()?, board.active_color)?;
            let dst_square = algebraic_to_coordinates(&r#move[2..])?;

            return piece_move(piece, dst_square, None, None, board);
        }

        // piece capture (row disambiguation)
        let re = Regex::new(PIECE_CAPTURE_ROW_DISAMBIGUATION_REGEX)
            .expect("Invalid piece capture row disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_algebraic_char(chars.next()?, board.active_color)?;
            let dst_square = algebraic_to_coordinates(&r#move[3..])?;
            let disambiguation_row = 7 - (chars.next()? as usize - 49);

            return piece_move(piece, dst_square, Some(disambiguation_row), None, board);
        }

        // piece capture (column disambiguation)
        let re = Regex::new(PIECE_CAPTURE_COLUMN_DISAMBIGUATION_REGEX)
            .expect("Invalid piece capture column disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_algebraic_char(chars.next()?, board.active_color)?;
            let dst_square = algebraic_to_coordinates(&r#move[3..])?;
            let disambiguation_column = chars.next()? as usize - 97;

            return piece_move(piece, dst_square, None, Some(disambiguation_column), board);
        }

        // piece capture (row and column disambiguation)
        let re = Regex::new(PIECE_CAPTURE_ROW_AND_COLUMN_DISAMBIGUATION_REGEX)
            .expect("Invalid piece capture row and column disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_algebraic_char(chars.next()?, board.active_color)?;
            let dst_square = algebraic_to_coordinates(&r#move[4..])?;
            let (disambiguation_row, disambiguation_column) =
                algebraic_to_coordinates(&r#move[1..3])?;

            return piece_move(
                piece,
                dst_square,
                Some(disambiguation_row),
                Some(disambiguation_column),
                board,
            );
        }

        // pawn promotion
        let re = Regex::new(PAWN_PROMOTION_REGEX).expect("Invalid pawn promotion regex");

        if re.is_match(r#move) {
            let dst_square = algebraic_to_coordinates(&r#move[0..2])?;
            let promotion_piece =
                Piece::from_algebraic_char(r#move.chars().nth(3)?, board.active_color)?;

            let mut r#move = pawn_move(dst_square, board);
            if let Some(ref mut r#move) = r#move {
                r#move.promotion = Some(promotion_piece);
            }

            return r#move;
        }

        // pawn capture promotion
        let re =
            Regex::new(PAWN_CAPTURE_PROMOTION_REGEX).expect("Invalid pawn capture promotion regex");

        if re.is_match(r#move) {
            let dst_square = algebraic_to_coordinates(&r#move[2..4]);
            let disambiguation = r#move.chars().nth(0)? as usize - 97;
            let promotion_piece =
                Piece::from_algebraic_char(r#move.chars().nth(5)?, board.active_color)?;

            let mut r#move = pawn_capture(dst_square?, disambiguation, board);
            if let Some(ref mut r#move) = r#move {
                r#move.promotion = Some(promotion_piece);
            }

            return r#move;
        }

        println!("Invalid move notation: {}", r#move);
        None
    }
}

pub fn castle(castle_kind: CastleKind, board: &Board) -> Option<Move> {
    match castle_kind {
        CastleKind::Kingside => match board.active_color {
            Color::White => {
                if !board.castle_rights.contains(&CastleRights::WhiteKingside)
                    || !board.square_attackers((7, 5)).is_empty()
                    || !board.square_attackers((7, 6)).is_empty()
                    || board.get_piece((7, 5)).is_some()
                    || board.get_piece((7, 6)).is_some()
                {
                    return None;
                }
            }
            Color::Black => {
                if !board.castle_rights.contains(&CastleRights::BlackKingside)
                    || !board.square_attackers((0, 5)).is_empty()
                    || !board.square_attackers((0, 6)).is_empty()
                    || board.get_piece((0, 5)).is_some()
                    || board.get_piece((0, 6)).is_some()
                {
                    return None;
                }
            }
        },

        CastleKind::Queenside => match board.active_color {
            Color::White => {
                if !board.castle_rights.contains(&CastleRights::WhiteQueenside)
                    || !board.square_attackers((7, 1)).is_empty()
                    || !board.square_attackers((7, 2)).is_empty()
                    || !board.square_attackers((7, 3)).is_empty()
                    || board.get_piece((7, 1)).is_some()
                    || board.get_piece((7, 2)).is_some()
                    || board.get_piece((7, 3)).is_some()
                {
                    return None;
                }
            }
            Color::Black => {
                if !board.castle_rights.contains(&CastleRights::BlackQueenside)
                    || !board.square_attackers((0, 1)).is_empty()
                    || !board.square_attackers((0, 2)).is_empty()
                    || !board.square_attackers((0, 3)).is_empty()
                    || board.get_piece((0, 1)).is_some()
                    || board.get_piece((0, 2)).is_some()
                    || board.get_piece((0, 3)).is_some()
                {
                    return None;
                }
            }
        },
    }

    Some(Move {
        src_square: None,
        dst_square: None,
        promotion: None,
        en_passant: None,
        en_passant_capture: false,
        castle: Some(castle_kind),
    })
}

pub fn pawn_move(dst_square: (usize, usize), board: &Board) -> Option<Move> {
    let dst_square_piece = board.get_piece(dst_square);

    for direction in PAWN_MOVE_DIRECTIONS.iter() {
        let src_square = match board.active_color {
            Color::Black => (
                (dst_square.0 as i8 - direction.0) as usize,
                (dst_square.1 as i8 - direction.1) as usize,
            ),
            Color::White => (
                (dst_square.0 as i8 + direction.0) as usize,
                (dst_square.1 as i8 + direction.1) as usize,
            ),
        };

        let src_square_piece = board.get_piece(src_square);

        // check if the move is a capture
        let is_capture = direction.1 != 0;

        // check for an invalid two square move
        let invalid_two_square_move = direction.0 == 2 && !(src_square.0 == 6 || src_square.0 == 1);

        let algo = match board.active_color {
            Color::Black => {
                if direction.0 == 2 {
                    board.get_piece((src_square.0 + 1, src_square.1)).is_some()
                } else {
                    false
                }
            }
            Color::White => {
                if direction.0 == 2 {
                    board.get_piece((src_square.0 - 1, src_square.1)).is_some()
                } else {
                    false
                }
            }
        };

        if is_capture
            || invalid_two_square_move
            || dst_square_piece.is_some()
            || src_square_piece != Some(Piece::Pawn(board.active_color))
            || algo
        {
            continue;
        }

        // check for en passant
        let en_passant = if direction.0 == 2 {
            match board.active_color {
                Color::Black => Some((dst_square.0 - 1, dst_square.1)),
                Color::White => Some((dst_square.0 + 1, dst_square.1)),
            }
        } else {
            None
        };

        let r#move = Move {
            src_square: Some(src_square),
            dst_square: Some(dst_square),
            promotion: None,
            en_passant,
            en_passant_capture: false,
            castle: None,
        };

        if board.future_check(&r#move) {
            continue;
        }

        return Some(r#move);
    }

    None
}

pub fn pawn_capture(
    dst_square: (usize, usize),
    disambiguation_column: usize,
    board: &Board,
) -> Option<Move> {
    let dst_square_piece = board.get_piece(dst_square);

    for direction in PAWN_CAPTURE_DIRECTIONS.iter() {
        let src_square = match board.active_color {
            Color::Black => (
                (dst_square.0 as i8 - direction.0) as usize,
                (dst_square.1 as i8 - direction.1) as usize,
            ),
            Color::White => (
                (dst_square.0 as i8 + direction.0) as usize,
                (dst_square.1 as i8 + direction.1) as usize,
            ),
        };

        let src_square_piece = board.get_piece(src_square);

        // check if the move is a capture
        let is_capture = direction.1 != 0;

        // check for an invalid capture
        // either the destination square is osccupied by a piece of the same
        // color or if it is empty, en passant is not possible or the en passant
        // square is not the same as the destination square
        let invalid_capture = dst_square_piece.is_some_and(|p| p.color() == &board.active_color)
            || dst_square_piece.is_none()
                && (board.en_passant.is_none()
                    || board.en_passant.is_some_and(|s| s != dst_square));

        // check for row disambiguation
        let invalid_disambiguation = src_square.1 != disambiguation_column;

        if !is_capture
            || invalid_capture
            || invalid_disambiguation
            || src_square_piece != Some(Piece::Pawn(board.active_color))
        {
            continue;
        }

        let en_passant_capture = board.en_passant.is_some_and(|s| s == dst_square);

        let r#move = Move {
            src_square: Some(src_square),
            dst_square: Some(dst_square),
            promotion: None,
            en_passant: None,
            en_passant_capture,
            castle: None,
        };

        if board.future_check(&r#move) {
            continue;
        }

        return Some(r#move);
    }

    None
}

pub fn piece_move(
    piece: Piece,
    dst_square: (usize, usize),
    disambiguation_row: Option<usize>,
    disambiguation_column: Option<usize>,
    board: &Board,
) -> Option<Move> {
    let move_directions = match piece {
        Piece::Knight(_) => KNIGHT_DIRECTIONS.to_vec(),
        Piece::Bishop(_) => BISHOP_DIRECTIONS.to_vec(),
        Piece::Rook(_) => ROOK_DIRECTIONS.to_vec(),
        Piece::Queen(_) => QUEEN_DIRECTIONS.to_vec(),
        Piece::King(_) => KING_DIRECTIONS.to_vec(),
        Piece::Pawn(_) => unreachable!(),
    };

    // check if the destination  square is occupied by a piece of the same color
    if board
        .get_piece(dst_square)
        .is_some_and(|p| p.color() == &board.active_color)
    {
        return None;
    }

    let mut valid_moves = vec![];

    for direction in move_directions.iter() {
        let mut src_square = (
            dst_square.0 as i8 + direction.0,
            dst_square.1 as i8 + direction.1,
        );

        // starting from the dst_square square, travel all the way in all possible directions
        // until we find the piece matching the one we are moving
        while (0..=7).contains(&src_square.0) && (0..=7).contains(&src_square.1) {
            let src_square_piece = board.get_piece((src_square.0 as usize, src_square.1 as usize));

            // if we find a piece it is blocking the way then we can stop looking in this direction
            if src_square_piece.is_some() && src_square_piece != Some(piece) {
                break;
            }

            // check for row disambiguation
            if let Some(row) = disambiguation_row {
                if row != src_square.0 as usize {
                    src_square.0 += direction.0;
                    src_square.1 += direction.1;
                    continue;
                }
            }

            // check for column disambiguation
            if let Some(column) = disambiguation_column {
                if column != src_square.1 as usize {
                    src_square.0 += direction.0;
                    src_square.1 += direction.1;
                    continue;
                }
            }

            // if the piece we are moving is not in the square we are looking
            // in, then go to the next square in the same direction if the
            // moving piece is not a queen, rook or bishop, or search in the
            // next direction if it is a queen, rook or bishop (queen, rook and
            // bishop can move multiple squares)
            if src_square_piece.is_none() {
                src_square.0 += direction.0;
                src_square.1 += direction.1;

                match piece {
                    Piece::Queen(_) => continue,
                    Piece::Rook(_) => continue,
                    Piece::Bishop(_) => continue,
                    Piece::Knight(_) => break,
                    Piece::King(_) => break,
                    Piece::Pawn(_) => unreachable!(),
                }
            }

            let r#move = Move {
                src_square: Some((src_square.0 as usize, src_square.1 as usize)),
                dst_square: Some(dst_square),
                promotion: None,
                en_passant: None,
                en_passant_capture: false,
                castle: None,
            };

            if board.future_check(&r#move) {
                src_square.0 += direction.0;
                src_square.1 += direction.1;
                match piece {
                    Piece::Queen(_) => continue,
                    Piece::Rook(_) => continue,
                    Piece::Bishop(_) => continue,
                    Piece::Knight(_) => break,
                    Piece::King(_) => break,
                    Piece::Pawn(_) => unreachable!(),
                }
            }
            valid_moves.push(r#move);

            src_square.0 += direction.0;
            src_square.1 += direction.1;

            break;
        }
    }

    match valid_moves.len() {
        0 => None,
        1 => {
            let r#move = valid_moves.first()?;
            Some(r#move.clone())
        }
        _ => {
            println!("Ambiguous move notation");
            None
        }
    }
}
