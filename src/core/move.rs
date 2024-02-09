use crate::constants::*;
use crate::core::{Board, CastleKind, Color, Piece, Square};

use regex::Regex;

/// Represents a chess move.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move {
    /// Color of the piece moving
    pub color: Color,

    /// Source square of the piece moving
    pub src_square: Option<Square>,

    /// Destination square of the piece moving
    pub dst_square: Option<Square>,

    /// Castle type
    pub castle: Option<CastleKind>,

    /// Piece to promote.
    pub promotion: Option<Piece>,
}

impl Move {
    /// Returns an UCI representation of the move.
    pub fn to_uci_str(&self) -> String {
        if let Some(castle) = self.castle {
            return castle.to_uci_str(&self.color);
        }

        // if the move is not a castle, it must have a source and destination
        // square so we can unwrap safely
        let src_square = self.src_square.unwrap();
        let dst_square = self.dst_square.unwrap();
        let promotion = match self.promotion {
            Some(piece) => piece.to_uci_char().to_string(),
            None => "".to_string(),
        };

        format!("{}-{}{}", src_square, dst_square, promotion)
    }

    /// Returns a [Move] struct representation of the given move in UCI
    /// notation.
    ///
    /// Either an UCI move with or without '-' will be accepted
    /// (e.g. "e2e4" or "e2-e4").
    pub fn from_uci(uci_str: &str, color: Color) -> Option<Move> {
        let re = Regex::new(UCI_MOVE_REGEX).expect("Invalid UCI move regex");
        let re_dash = Regex::new(UCI_MOVE_DASH_REGEX).expect("Invalid UCI move dash regex");

        let dash_uci = re_dash.is_match(uci_str);
        if !re.is_match(uci_str) && !dash_uci {
            return None;
        }

        let (src_square_str, dst_square_str, promotion_char) = match dash_uci {
            true => (&uci_str[0..2], &uci_str[3..5], uci_str.chars().nth(5)),
            false => (&uci_str[0..2], &uci_str[2..4], uci_str.chars().nth(4)),
        };

        let src_square = Square::from_san_str(src_square_str)?;
        let dst_square = Square::from_san_str(dst_square_str)?;
        let castle = CastleKind::from_uci_str(uci_str);
        let promotion = match promotion_char {
            Some(char) => Some(Piece::from_uci_char(char, color)?),
            None => None,
        };

        match castle {
            Some(castle_type) => Some(Move {
                color,
                src_square: None,
                dst_square: None,
                castle: Some(castle_type),
                promotion: None,
            }),
            None => Some(Move {
                color,
                src_square: Some(src_square),
                dst_square: Some(dst_square),
                castle: None,
                promotion,
            }),
        }
    }

    /// Returns a [Move] struct representation of the given move in standard
    /// algebraic notation. Will return a move when it is valid even if it
    /// is illegal.
    pub fn from_san(r#move: &str, board: &Board) -> Option<Move> {
        // castling
        let re = Regex::new(CASTLE_REGEX).expect("Invalid castle regex");

        if re.is_match(r#move) {
            let castle_type = CastleKind::from_san_str(r#move)?;
            return Some(Move {
                color: board.active_color,
                src_square: None,
                dst_square: None,
                castle: Some(castle_type),
                promotion: None,
            });
        };

        // pawn move
        let re = Regex::new(PAWN_MOVE_REGEX).expect("Invalid pawn move regex");

        if re.is_match(r#move) {
            let dst_square = Square::from_san_str(r#move)?;
            return algebraic_piece_move(
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
            let piece = Piece::from_san_char(r#move.chars().next()?, board.active_color)?;
            let dst_square = Square::from_san_str(&r#move[1..])?;

            return algebraic_piece_move(piece, dst_square, None, None, board);
        }

        // piece move row disambiguation
        let re = Regex::new(PIECE_MOVE_ROW_DISAMBIGUATION_REGEX)
            .expect("Invalid piece move row disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_san_char(chars.next()?, board.active_color)?;
            let dst_square = Square::from_san_str(&r#move[2..])?;
            let disambiguation_row = 7 - (chars.next()? as usize - 49);

            return algebraic_piece_move(piece, dst_square, Some(disambiguation_row), None, board);
        }

        // piece move column disambiguation
        let re = Regex::new(PIECE_MOVE_COLUMN_DISAMBIGUATION_REGEX)
            .expect("Invalid piece move column disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_san_char(chars.next().unwrap(), board.active_color)?;
            let dst_square = Square::from_san_str(&r#move[2..])?;
            let disambiguation_column = chars.next()? as usize - 97;

            return algebraic_piece_move(
                piece,
                dst_square,
                None,
                Some(disambiguation_column),
                board,
            );
        }

        // piece move row and column disambiguation
        let re = Regex::new(PIECE_MOVE_ROW_AND_COLUMN_DISAMBIGUATION_REGEX)
            .expect("Invalid piece move row and column disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_san_char(chars.next()?, board.active_color)?;
            let dst_square = Square::from_san_str(&r#move[3..])?;
            let src_square = Square::from_san_str(&r#move[1..3])?;

            return algebraic_piece_move(
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
            let dst_square = Square::from_san_str(&r#move[2..])?;
            let disambiguation_column = r#move.chars().nth(0)? as usize - 97;

            return algebraic_piece_move(
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
            let piece = Piece::from_san_char(chars.next()?, board.active_color)?;
            let dst_square = Square::from_san_str(&r#move[2..])?;

            return algebraic_piece_move(piece, dst_square, None, None, board);
        }

        // piece capture row disambiguation
        let re = Regex::new(PIECE_CAPTURE_ROW_DISAMBIGUATION_REGEX)
            .expect("Invalid piece capture row disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_san_char(chars.next()?, board.active_color)?;
            let dst_square = Square::from_san_str(&r#move[3..])?;
            let disambiguation_row = 7 - (chars.next()? as usize - 49);

            return algebraic_piece_move(piece, dst_square, Some(disambiguation_row), None, board);
        }

        // piece capture column disambiguation
        let re = Regex::new(PIECE_CAPTURE_COLUMN_DISAMBIGUATION_REGEX)
            .expect("Invalid piece capture column disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_san_char(chars.next()?, board.active_color)?;
            let dst_square = Square::from_san_str(&r#move[3..])?;
            let disambiguation_column = chars.next()? as usize - 97;

            return algebraic_piece_move(
                piece,
                dst_square,
                None,
                Some(disambiguation_column),
                board,
            );
        }

        // piece capture row and column disambiguation
        let re = Regex::new(PIECE_CAPTURE_ROW_AND_COLUMN_DISAMBIGUATION_REGEX)
            .expect("Invalid piece capture row and column disambiguation regex");

        if re.is_match(r#move) {
            let mut chars = r#move.chars();
            let piece = Piece::from_san_char(chars.next()?, board.active_color)?;
            let dst_square = Square::from_san_str(&r#move[4..])?;
            let src_square = Square::from_san_str(&r#move[1..3])?;

            return algebraic_piece_move(
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
            let dst_square = Square::from_san_str(&r#move[0..2])?;
            let promotion_piece = Piece::from_san_char(r#move.chars().nth(3)?, board.active_color)?;

            let mut r#move = algebraic_piece_move(
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
            let dst_square = Square::from_san_str(&r#move[2..4])?;
            let disambiguation = r#move.chars().nth(0)? as usize - 97;
            let promotion_piece = Piece::from_san_char(r#move.chars().nth(5)?, board.active_color)?;

            let mut r#move = algebraic_piece_move(
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

/// Returns a move from algebraic notation data.
fn algebraic_piece_move(
    piece: Piece,
    dst_square: Square,
    disambiguation_row: Option<usize>,
    disambiguation_column: Option<usize>,
    board: &Board,
) -> Option<Move> {
    // handle pawn moves separately
    if let Piece::Pawn(_) = piece {
        return algebraic_pawn_move(dst_square, board, disambiguation_column);
    }

    let mut valid_moves = vec![];
    for direction in piece.directions().iter() {
        let mut src_square = Square(
            (dst_square.0 as i8 + direction.0) as usize,
            (dst_square.1 as i8 + direction.1) as usize,
        );

        // starting from the dst_square square, travel all the way in all possible
        // directions until we find the piece matching the one we are moving
        while (0..=7).contains(&src_square.0) && (0..=7).contains(&src_square.1) {
            let src_square_piece = board.get_piece(src_square);

            // if we find a piece it is blocking the way then we can stop looking in this
            // direction
            if src_square_piece.is_some_and(|p| p != piece) {
                break;
            }

            // check for row disambiguation
            if let Some(row) = disambiguation_row {
                if row != src_square.0 {
                    src_square += direction;
                    continue;
                }
            }

            // check for column disambiguation
            if let Some(column) = disambiguation_column {
                if column != src_square.1 {
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
                color: board.active_color,
                src_square: Some(src_square),
                dst_square: Some(dst_square),
                promotion: None,
                castle: None,
            };

            // we need this in order to prevent false disambiguation when one of two pieces
            // that can move to the same square is pinned.
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
            Some(*r#move)
        }
        _ => {
            println!("Ambiguous move notation");
            None
        }
    }
}

/// Returns a pawn move from algebraic notation data.
fn algebraic_pawn_move(
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

        // if the source square is out of bounds, skip and continue with the next
        // direction
        if !(0..=7).contains(&src_square.0) || !(0..=7).contains(&src_square.1) {
            continue;
        }

        let src_square_piece = board.get_piece(src_square);

        // if the source square is empty, or it is not the piece we are moving, skip and
        // continue with the next direction
        if src_square_piece.is_some_and(|p| p != piece) || src_square_piece.is_none() {
            continue;
        }

        // check for column disambiguation
        if let Some(column) = disambiguation_column {
            if column != src_square.1 {
                continue;
            }
        }

        return Some(Move {
            color: board.active_color,
            src_square: Some(src_square),
            dst_square: Some(dst_square),
            promotion: None,
            castle: None,
        });
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_move_from_uci_notation() {
        // normal pawn move
        let r#move = Move::from_uci("e2e4", Color::White);
        assert_eq!(
            r#move,
            Some(Move {
                color: Color::White,
                src_square: Some(Square(6, 4)),
                dst_square: Some(Square(4, 4)),
                promotion: None,
                castle: None,
            })
        );

        // white kingside castle
        let r#move = Move::from_uci("e1g1", Color::White);
        assert_eq!(
            r#move,
            Some(Move {
                color: Color::White,
                src_square: None,
                dst_square: None,
                promotion: None,
                castle: Some(CastleKind::Kingside),
            })
        );

        // promotion
        let r#move = Move::from_uci("e7e8q", Color::Black);
        assert_eq!(
            r#move,
            Some(Move {
                color: Color::Black,
                src_square: Some(Square(1, 4)),
                dst_square: Some(Square(0, 4)),
                promotion: Some(Piece::Queen(Color::Black)),
                castle: None,
            })
        );
    }
}
