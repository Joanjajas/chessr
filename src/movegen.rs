use crate::board::Board;
use crate::color::Color;
use crate::constants::*;
use crate::piece::Piece;
use crate::r#move::Move;

/// Generate all legal pawn moves for the current position
pub fn pawn_legal_moves(src_square: (usize, usize), board: &Board) -> Vec<Move> {
    let mut legal_moves = Vec::new();

    // Pawn forward move
    for direction in PAWN_MOVE_DIRECTIONS.iter() {
        let dst_square = match board.active_color {
            Color::Black => (
                (src_square.0 as i8 + direction.0) as usize,
                (src_square.1 as i8 - direction.1) as usize,
            ),
            Color::White => (
                (src_square.0 as i8 - direction.0) as usize,
                (src_square.1 as i8 + direction.1) as usize,
            ),
        };

        let dst_square_piece = board.get_piece(dst_square);
        if dst_square_piece.is_some() {
            continue;
        }

        if direction.0 == 2 && !(src_square.0 == 6 || src_square.0 == 1) {
            continue;
        }

        match board.active_color {
            Color::Black => {
                if direction.0 == 2 && board.get_piece((src_square.0 + 1, src_square.1)).is_some() {
                    continue;
                }
            }
            Color::White => {
                if direction.0 == 2 && board.get_piece((src_square.0 - 1, src_square.1)).is_some() {
                    continue;
                }
            }
        };

        let r#move = Move {
            src_square: Some(src_square),
            dst_square: Some(dst_square),
            promotion: None,
            en_passant: None,
            en_passant_capture: false,
            castle: None,
        };

        if board.future_check(&r#move) {
            continue;
        }

        legal_moves.push(r#move);
    }

    // Pawn capture
    for direction in PAWN_CAPTURE_DIRECTIONS.iter() {
        let dst_square = match board.active_color {
            Color::Black => (
                (src_square.0 as i8 + direction.0) as usize,
                (src_square.1 as i8 - direction.1) as usize,
            ),
            Color::White => (
                (src_square.0 as i8 - direction.0) as usize,
                (src_square.1 as i8 + direction.1) as usize,
            ),
        };

        let en_passant_capture = board.en_passant.is_some_and(|s| s == dst_square);

        let dst_square_piece = board.get_piece(dst_square);
        if (dst_square_piece.is_none() && !en_passant_capture)
            || dst_square_piece.is_some_and(|p| p.color() == &board.active_color)
        {
            continue;
        }

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

        legal_moves.push(r#move);
    }

    legal_moves
}

/// Generate all legal piece moves for the current position
pub fn piece_legal_moves(piece: &Piece, src_square: (usize, usize), board: &Board) -> Vec<Move> {
    let mut legal_moves = Vec::new();

    let move_directions = match piece {
        Piece::Knight(_) => KNIGHT_DIRECTIONS.to_vec(),
        Piece::Bishop(_) => BISHOP_DIRECTIONS.to_vec(),
        Piece::Rook(_) => ROOK_DIRECTIONS.to_vec(),
        Piece::Queen(_) => QUEEN_DIRECTIONS.to_vec(),
        Piece::King(_) => KING_DIRECTIONS.to_vec(),
        Piece::Pawn(_) => unreachable!(),
    };

    for direction in move_directions.iter() {
        let mut dst_square = (
            src_square.0 as i8 + direction.0,
            src_square.1 as i8 + direction.1,
        );

        while (0..=7).contains(&dst_square.0) && (0..=7).contains(&dst_square.1) {
            let dst_square_piece = board.get_piece((dst_square.0 as usize, dst_square.1 as usize));

            if dst_square_piece.is_some_and(|p| p.color() != &board.active_color) {
                let r#move = Move {
                    src_square: Some(src_square),
                    dst_square: Some((dst_square.0 as usize, dst_square.1 as usize)),
                    promotion: None,
                    en_passant: None,
                    en_passant_capture: false,
                    castle: None,
                };
                if !board.future_check(&r#move) {
                    legal_moves.push(r#move);
                }

                break;
            }

            if dst_square_piece.is_some_and(|p| p.color() == &board.active_color) {
                break;
            }

            let r#move = Move {
                src_square: Some(src_square),
                dst_square: Some((dst_square.0 as usize, dst_square.1 as usize)),
                promotion: None,
                en_passant: None,
                en_passant_capture: false,
                castle: None,
            };

            if board.future_check(&r#move) {
                dst_square.0 += direction.0;
                dst_square.1 += direction.1;
                match piece {
                    Piece::Queen(_) => continue,
                    Piece::Rook(_) => continue,
                    Piece::Bishop(_) => continue,
                    Piece::Knight(_) => break,
                    Piece::King(_) => break,
                    Piece::Pawn(_) => unreachable!(),
                }
            }

            legal_moves.push(r#move);

            dst_square.0 += direction.0;
            dst_square.1 += direction.1;

            match piece {
                Piece::Queen(_) => continue,
                Piece::Rook(_) => continue,
                Piece::Bishop(_) => continue,
                Piece::Knight(_) => break,
                Piece::King(_) => break,
                Piece::Pawn(_) => unreachable!(),
            }
        }
    }

    legal_moves
}
