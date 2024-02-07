use crate::constants::*;
use crate::core::{Board, CastleKind, Color, Piece, Square};

use regex::Regex;

/// Represents a chess move.
#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    /// Source square of the piece moving
    pub src_square: Option<Square>,

    /// Destination square of the piece moving
    pub dst_square: Option<Square>,

    /// En passant target square
    pub en_passant: Option<Square>,

    /// Whether the move is an en passant capture
    pub en_passant_capture: bool,

    /// Castle type
    pub castle: Option<CastleKind>,

    /// Piece to promote.
    pub promotion: Option<Piece>,
}

impl Move {
    /// Returns a [Move] struct representation of the given algebraic notation for the given board.
    /// Will return a move when it is valid even if it is illegal.
    pub fn from_algebraic(r#move: &str, board: &Board) -> Option<Move> {
        // castling
        let re = Regex::new(CASTLE_REGEX).expect("Invalid castle regex");

        if re.is_match(r#move) {
            let castle_type = CastleKind::from_algebraic(r#move)?;
            return Some(Move {
                src_square: None,
                dst_square: None,
                en_passant: None,
                en_passant_capture: false,
                castle: Some(castle_type),
                promotion: None,
            });
        };

        // pawn move
        let re = Regex::new(PAWN_MOVE_REGEX).expect("Invalid pawn move regex");

        if re.is_match(r#move) {
            let dst_square = Square::from_algebraic(r#move)?;
            return piece_move(
                Piece::Pawn(board.active_color),
                dst_square,
                None,
                None,
                board,
            );
        }

        // piece move
        let re = Regex::new(PIECE_MOVE_REGEX).expect("Invalid piece move regex");

        if re.is_match(r#move) {
            let piece = Piece::from_algebraic_char(r#move.chars().next()?, board.active_color)?;
            let dst_square = Square::from_algebraic(&r#move[1..])?;

            return piece_move(piece, dst_square, None, None, board);
        }

        // piece move row disambiguation
        let re = Regex::new(PIECE_MOVE_ROW_DISAMBIGUATION_REGEX)
            .expect("Invalid piece move row disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_algebraic_char(chars.next()?, board.active_color)?;
            let dst_square = Square::from_algebraic(&r#move[2..])?;
            let disambiguation_row = 7 - (chars.next()? as usize - 49);

            return piece_move(piece, dst_square, Some(disambiguation_row), None, board);
        }

        // piece move column disambiguation
        let re = Regex::new(PIECE_MOVE_COLUMN_DISAMBIGUATION_REGEX)
            .expect("Invalid piece move column disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_algebraic_char(chars.next().unwrap(), board.active_color)?;
            let dst_square = Square::from_algebraic(&r#move[2..])?;
            let disambiguation_column = chars.next()? as usize - 97;

            return piece_move(piece, dst_square, None, Some(disambiguation_column), board);
        }

        // piece move row and column disambiguation
        let re = Regex::new(PIECE_MOVE_ROW_AND_COLUMN_DISAMBIGUATION_REGEX)
            .expect("Invalid piece move row and column disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_algebraic_char(chars.next()?, board.active_color)?;
            let dst_square = Square::from_algebraic(&r#move[3..])?;
            let src_square = Square::from_algebraic(&r#move[1..3])?;

            return piece_move(
                piece,
                dst_square,
                Some(src_square.0),
                Some(src_square.1),
                board,
            );
        }

        // pawn capture
        let re = Regex::new(PAWN_CAPTURE_REGEX).expect("Invalid pawn capture regex");

        if re.is_match(r#move) {
            let dst_square = Square::from_algebraic(&r#move[2..])?;
            let disambiguation_column = r#move.chars().nth(0)? as usize - 97;

            return piece_move(
                Piece::Pawn(board.active_color),
                dst_square,
                None,
                Some(disambiguation_column),
                board,
            );
        }

        // piece capture
        let re = Regex::new(PIECE_CAPTURE_REGEX).expect("Invalid piece capture regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_algebraic_char(chars.next()?, board.active_color)?;
            let dst_square = Square::from_algebraic(&r#move[2..])?;

            return piece_move(piece, dst_square, None, None, board);
        }

        // piece capture row disambiguation
        let re = Regex::new(PIECE_CAPTURE_ROW_DISAMBIGUATION_REGEX)
            .expect("Invalid piece capture row disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_algebraic_char(chars.next()?, board.active_color)?;
            let dst_square = Square::from_algebraic(&r#move[3..])?;
            let disambiguation_row = 7 - (chars.next()? as usize - 49);

            return piece_move(piece, dst_square, Some(disambiguation_row), None, board);
        }

        // piece capture column disambiguation
        let re = Regex::new(PIECE_CAPTURE_COLUMN_DISAMBIGUATION_REGEX)
            .expect("Invalid piece capture column disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_algebraic_char(chars.next()?, board.active_color)?;
            let dst_square = Square::from_algebraic(&r#move[3..])?;
            let disambiguation_column = chars.next()? as usize - 97;

            return piece_move(piece, dst_square, None, Some(disambiguation_column), board);
        }

        // piece capture row and column disambiguation
        let re = Regex::new(PIECE_CAPTURE_ROW_AND_COLUMN_DISAMBIGUATION_REGEX)
            .expect("Invalid piece capture row and column disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_algebraic_char(chars.next()?, board.active_color)?;
            let dst_square = Square::from_algebraic(&r#move[4..])?;
            let src_square = Square::from_algebraic(&r#move[1..3])?;

            return piece_move(
                piece,
                dst_square,
                Some(src_square.0),
                Some(src_square.1),
                board,
            );
        }

        // pawn promotion
        let re = Regex::new(PAWN_PROMOTION_REGEX).expect("Invalid pawn promotion regex");

        if re.is_match(r#move) {
            let dst_square = Square::from_algebraic(&r#move[0..2])?;
            let promotion_piece =
                Piece::from_algebraic_char(r#move.chars().nth(3)?, board.active_color)?;

            let mut r#move = piece_move(
                Piece::Pawn(board.active_color),
                dst_square,
                None,
                None,
                board,
            );

            if let Some(ref mut r#move) = r#move {
                r#move.promotion = Some(promotion_piece);
            }

            return r#move;
        }

        // pawn capture promotion
        let re =
            Regex::new(PAWN_CAPTURE_PROMOTION_REGEX).expect("Invalid pawn capture promotion regex");

        if re.is_match(r#move) {
            let dst_square = Square::from_algebraic(&r#move[2..4])?;
            let disambiguation = r#move.chars().nth(0)? as usize - 97;
            let promotion_piece =
                Piece::from_algebraic_char(r#move.chars().nth(5)?, board.active_color)?;

            let mut r#move = piece_move(
                Piece::Pawn(board.active_color),
                dst_square,
                None,
                Some(disambiguation),
                board,
            );

            if let Some(ref mut r#move) = r#move {
                r#move.promotion = Some(promotion_piece);
            }

            return r#move;
        }

        None
    }
}

pub fn piece_move(
    piece: Piece,
    dst_square: Square,
    disambiguation_row: Option<usize>,
    disambiguation_column: Option<usize>,
    board: &Board,
) -> Option<Move> {
    // handle pawn moves separately
    if let Piece::Pawn(_) = piece {
        return pawn_move(dst_square, board, disambiguation_column);
    }

    let mut valid_moves = vec![];
    for direction in piece.directions().iter() {
        let mut src_square = Square(
            (dst_square.0 as i8 + direction.0) as usize,
            (dst_square.1 as i8 + direction.1) as usize,
        );

        // starting from the dst_square square, travel all the way in all possible directions
        // until we find the piece matching the one we are moving
        while (0..=7).contains(&src_square.0) && (0..=7).contains(&src_square.1) {
            let src_square_piece = board.get_piece(src_square);

            // if we find a piece it is blocking the way then we can stop looking in this direction
            if src_square_piece.is_some_and(|p| p != piece) {
                break;
            }

            // check for row disambiguation
            if let Some(row) = disambiguation_row {
                if row != src_square.0 as usize {
                    src_square += direction;
                    continue;
                }
            }

            // check for column disambiguation
            if let Some(column) = disambiguation_column {
                if column != src_square.1 as usize {
                    src_square += direction;
                    continue;
                }
            }

            // if the source square is empty, depending on the piece type we can continue
            // looking in the same direction or skip to the next direction
            if src_square_piece.is_none() {
                src_square += direction;

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
                src_square: Some(src_square),
                dst_square: Some(dst_square),
                promotion: None,
                en_passant: None,
                en_passant_capture: false,
                castle: None,
            };

            if !board.future_check(&r#move) {
                valid_moves.push(r#move);
            }

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

pub fn pawn_move(
    dst_square: Square,
    board: &Board,
    disambiguation_column: Option<usize>,
) -> Option<Move> {
    let piece = Piece::Pawn(board.active_color);

    for direction in piece.directions() {
        let src_square = Square(
            (dst_square.0 as i8 - direction.0) as usize,
            (dst_square.1 as i8 - direction.1) as usize,
        );

        // if the source square is out of bounds, skip and continue with the next direction
        if !(0..=7).contains(&src_square.0) || !(0..=7).contains(&src_square.1) {
            continue;
        }

        let src_square_piece = board.get_piece(src_square);

        // if the source square is empty, or it is not the piece we are moving, skip and continue with the next direction
        if src_square_piece.is_some_and(|p| p != piece) || src_square_piece.is_none() {
            continue;
        }

        // check for column disambiguation
        if let Some(column) = disambiguation_column {
            if column != src_square.1 as usize {
                continue;
            }
        }

        // check for en passant
        let en_passant_capture = board.en_passant.is_some_and(|s| s == dst_square);
        let en_passant = if direction.0 == 2 {
            match board.active_color {
                Color::Black => Some((dst_square.0 - 1, dst_square.1).into()),
                Color::White => Some((dst_square.0 + 1, dst_square.1).into()),
            }
        } else {
            None
        };

        return Some(Move {
            src_square: Some(src_square),
            dst_square: Some(dst_square),
            promotion: None,
            en_passant,
            en_passant_capture,
            castle: None,
        });
    }

    None
}
