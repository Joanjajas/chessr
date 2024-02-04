use crate::board::Board;
use crate::castle::CastleKind;
use crate::color::Color;
use crate::piece::Piece;
use crate::r#move::{self, Move};

/// Returns a vec of [Move] containing all possible legal moves in the current position.
pub fn generate_legal_moves(board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();

    // piece moves
    for (row, col) in board.pieces.iter().enumerate() {
        for (col, piece) in col.iter().enumerate() {
            if piece.is_some_and(|p| p.color() != &board.active_color) || piece.is_none() {
                continue;
            }

            let mut legal_moves = legal_moves(&piece.unwrap(), (row, col), board);
            moves.append(&mut legal_moves);
        }
    }

    // kingside castling
    if let Some(castle) = r#move::castle(CastleKind::Kingside, board) {
        moves.push(castle);
    }

    // queenside castling
    if let Some(castle) = r#move::castle(CastleKind::Queenside, board) {
        moves.push(castle);
    }

    moves
}

/// Returns a vec of [Move] containing all possible legal moves for the given piece in the current position.
fn legal_moves(piece: &Piece, src_square: (usize, usize), board: &Board) -> Vec<Move> {
    let mut legal_moves = Vec::new();

    // handle pawn moves separately
    if let Piece::Pawn(_) = piece {
        return pawn_legal_moves(src_square, board);
    }

    for direction in piece.directions().iter() {
        let mut dst_square = (
            src_square.0 as i8 + direction.0,
            src_square.1 as i8 + direction.1,
        );

        while (0..=7).contains(&dst_square.0) && (0..=7).contains(&dst_square.1) {
            let dst_square_piece = board.get_piece((dst_square.0 as usize, dst_square.1 as usize));

            // if the piece is the same color, we can't move there or beyond
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

            // if the piece is the opposite color, we can move there and take it, but not beyond
            if dst_square_piece.is_some_and(|p| p.color() != &board.active_color) {
                if !board.future_check(&r#move) {
                    legal_moves.push(r#move);
                }

                break;
            }

            // if the square is empty don't move our king into check or move a pinned piece
            if !board.future_check(&r#move) {
                legal_moves.push(r#move);
            }

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

/// Returns a vec of [Move] containing all possible legal moves for the ginve pawn in the current position.
fn pawn_legal_moves(src_square: (usize, usize), board: &Board) -> Vec<Move> {
    let mut legal_moves = Vec::new();
    let piece = Piece::Pawn(board.active_color);

    // we have 3 different kind of moves: forward, two square and capture.
    // depending on the color of the pawn the direction is positive or negative.
    for direction in piece.directions().iter() {
        let dst_square = (
            (src_square.0 as i8 + direction.0) as usize,
            (src_square.1 as i8 + direction.1) as usize,
        );
        let dst_square_piece = board.get_piece(dst_square);

        // check if is a forward move and is valid
        let invalid_forward_move = direction.1 == 0 && dst_square_piece.is_some();

        // check if is a two square move and is valid
        let invalid_two_square_move_row = src_square.0 != 6 && src_square.0 != 1;
        let piece_blocking_two_square_move = match board.active_color {
            Color::Black => board.get_piece((dst_square.0 - 1, dst_square.1)).is_some(),
            Color::White => board.get_piece((dst_square.0 + 1, dst_square.1)).is_some(),
        };
        let invalid_two_square_move = (direction.0 == 2 || direction.0 == -2)
            && (invalid_two_square_move_row
                || piece_blocking_two_square_move
                || dst_square_piece.is_some());

        // check if is a capture move and is valid
        let invalid_en_passant =
            board.en_passant.is_some_and(|s| s != dst_square) || board.en_passant.is_none();
        let invalid_capture = direction.1 != 0
            && (dst_square_piece.is_none() && invalid_en_passant)
            || dst_square_piece.is_some_and(|p| p.color() == &board.active_color);

        // if one of the conditions is met, skip and continue with the next direction
        if invalid_forward_move || invalid_two_square_move || invalid_capture {
            continue;
        }

        let r#move = Move {
            src_square: Some(src_square),
            dst_square: Some(dst_square),
            promotion: None,
            en_passant: None,
            en_passant_capture: false,
            castle: None,
        };

        // don't move the pawn if it is pinned
        if !board.future_check(&r#move) {
            legal_moves.push(r#move);
        }
    }

    legal_moves
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_legal_moves() {
        // initial position
        let mut board =
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        assert_eq!(board.legal_moves().len(), 20);

        // checkmate
        board = Board::from_fen("8/5pk1/6p1/8/5P1Q/1b6/q7/K7 w - - 12 50").unwrap();
        assert_eq!(board.legal_moves().len(), 0);

        // stalemate
        board = Board::from_fen("8/7p/8/8/1p6/5k2/5p2/5K2 w - - 4 56").unwrap();
        assert_eq!(board.legal_moves().len(), 0);

        // check
        board = Board::from_fen("4R1k1/ppp2ppp/2b5/8/3P1B2/P4N2/2P2PPP/6K1 b - - 0 20").unwrap();
        assert_eq!(board.legal_moves().len(), 1);

        board = Board::from_fen("rnb2rk1/ppp2ppp/3p1n2/8/3PP3/P1P2N2/2P2PPP/R1B1KB1R b KQ - 0 9")
            .unwrap();
        assert_eq!(board.legal_moves().len(), 28);

        board =
            Board::from_fen("rnb1kbnr/p1pp1ppp/1p6/4p1q1/2B1P3/P7/1PPP1PPP/RNBQK1NR w KQkq - 2 4")
                .unwrap();
        assert_eq!(board.legal_moves().len(), 33);
    }

    #[test]
    fn test_pawn_legal_moves() {
        // frontal pinned pawn
        let mut board =
            Board::from_fen("rnb1kbnr/ppp1pppp/4q3/3p4/P3P3/8/1PPP1PPP/RNBQKBNR w KQkq - 1 4")
                .unwrap();
        assert_eq!(pawn_legal_moves((4, 4), &board).len(), 1);

        // diagonal pinned pawn
        board = Board::from_fen("rnb1kbnr/ppp1pppp/8/q2p4/4P3/8/1PPP1PPP/RNBQKBNR w KQkq - 0 5")
            .unwrap();
        assert_eq!(pawn_legal_moves((6, 3), &board).len(), 0);

        // en passant
        board = Board::from_fen("rnbqkbnr/1pp1pppp/p7/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3")
            .unwrap();
        assert_eq!(pawn_legal_moves((3, 4), &board).len(), 2);

        // blocking pawn (one square move)
        board =
            Board::from_fen("rnbqkbnr/1ppppppp/8/p7/P7/8/1PPPPPPP/RNBQKBNR w KQkq - 0 2").unwrap();
        assert_eq!(pawn_legal_moves((4, 0), &board).len(), 0);

        // blocking pawn (two square move)
        board =
            Board::from_fen("rnbqkbnr/1ppppppp/p7/8/P7/8/1PPPPPPP/RNBQKBNR w KQkq - 0 2").unwrap();
        assert_eq!(pawn_legal_moves((4, 0), &board).len(), 1);

        // capture
        board = Board::from_fen("rn2kbnr/pppqp1pp/8/3p1p2/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 5")
            .unwrap();
        assert_eq!(pawn_legal_moves((4, 4), &board).len(), 3);
    }

    #[test]
    fn test_piece_legal_moves() {
        // king can't move
        let mut board = Board::from_fen("R7/2p5/8/2k3p1/1r6/K1P5/PP6/8 w - - 6 43").unwrap();
        assert_eq!(
            legal_moves(&Piece::King(Color::White), (5, 0), &board).len(),
            0
        );

        // king under check
        board = Board::from_fen("5R2/2p5/8/2k3p1/r7/K1P5/PP6/8 w - - 8 44").unwrap();
        assert_eq!(
            legal_moves(&Piece::King(Color::White), (5, 0), &board).len(),
            2
        );

        // pinned piece
        board = Board::from_fen("rnbqk1nr/1pppbppp/p7/8/4QB2/P7/1PP1PPPP/RN2KBNR b KQkq - 3 5")
            .unwrap();
        assert_eq!(
            legal_moves(&Piece::Bishop(Color::Black), (1, 4), &board).len(),
            0
        );
    }
}
