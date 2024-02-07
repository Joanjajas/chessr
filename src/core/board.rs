use crate::constants::FEN_STARTING_POSITION;
use crate::core::{movegen, CastleKind, CastleRights, Color, Move, Piece, Square};
use crate::fen::{self, FenParseError};

/// Represents a chess board.
#[derive(Debug, Clone)]
pub struct Board {
    /// Piece placement representation on the board.
    pub pieces: [[Option<Piece>; 8]; 8],

    /// Color of the player who moves next.
    pub active_color: Color,

    /// Castling availability for each player and castle type
    pub castle_rights: Vec<CastleRights>,

    /// En passant target square.
    pub en_passant: Option<Square>,

    /// Number of moves since the last capture or pawn advance.
    pub halfmove_clock: u32,

    /// Number of completed turns in the game.
    pub fullmove_number: u32,
}

impl Board {
    /// Creates a new board with the starting position.
    pub fn new() -> Board {
        fen::fen_to_board(FEN_STARTING_POSITION).unwrap()
    }

    /// Creates a FEN string representation of the current board.
    /// [Forsyth–Edwards Notation](https://www.chess.com/terms/fen-chess) (FEN) is a standard notation for describing a particular board position of a chess game.
    pub fn from_fen(fen_str: &str) -> Result<Board, FenParseError> {
        fen::fen_to_board(fen_str)
    }

    /// Creates a FEN string representation of the current the board.
    /// [Forsyth–Edwards Notation](https://www.chess.com/terms/fen-chess) (FEN) is a standard notation for describing a particular board position of a chess game.
    pub fn fen(&self) -> String {
        fen::board_to_fen(self)
    }

    /// Returns true if there is a check in the current position.
    pub fn check(&self) -> bool {
        !self.square_attackers(self.king_square()).is_empty()
    }

    /// Returns true if there is a checkmate in the current position.
    pub fn checkmate(&self) -> bool {
        self.check() && self.legal_moves().is_empty()
    }

    /// Returns true if there is a stalemate in the current position.
    pub fn stalemate(&self) -> bool {
        !self.check() && self.legal_moves().is_empty()
    }

    /// Returns true if 50 moves have been made without a pawn move or a capture.
    pub fn fifty_move_rule(&self) -> bool {
        self.halfmove_clock >= 100
    }

    /// Makes a move on the board given its [algebraic notation](https://www.chess.com/terms/chess-notation).
    /// If the move notation is invalid or the move is not legal, nothing will happen.
    /// Also returns the move that was made.
    pub fn make_move_algebraic(&mut self, algebraic_move: &str) -> Option<Move> {
        let r#move = Move::from_algebraic(algebraic_move, self);

        if let Some(ref r#move) = r#move {
            if self.legal_moves().contains(r#move) {
                self.make_move(r#move);
            }
        }

        r#move
    }

    /// Returns a vec of [Move] containing all possible legal moves in the current position.
    pub fn legal_moves(&self) -> Vec<Move> {
        movegen::generate_legal_moves(self)
    }

    /// Returns the piece located at the given square, if any.
    /// If the square provided is out of bounds, the method will panic.
    pub(crate) fn get_piece(&self, square: Square) -> Option<Piece> {
        self.pieces[square.0][square.1]
    }

    /// Sets the piece at the given square.
    /// To remove a piece from a square, pass `None` as the piece.
    /// If the square provided is out of bounds, the method will panic.
    pub(crate) fn set_piece(&mut self, square: Square, piece: Option<Piece>) {
        self.pieces[square.0][square.1] = piece;
    }

    /// Makes a move on the board, updating the board state.
    /// This method assumes that the move is legal and valid, otherwise undefined behavior may occur.
    pub(crate) fn make_move(&mut self, r#move: &Move) {
        // handle castling
        if let Some(ref castle) = r#move.castle {
            match castle {
                CastleKind::Kingside => self.castle_kingside(),
                CastleKind::Queenside => self.castle_queenside(),
            }
        }

        // handle normal move
        if let (Some(src_square), Some(dst_square)) = (r#move.src_square, r#move.dst_square) {
            // handle en pasant capture
            let en_passant_capture = self.en_passant.is_some_and(|s| s == dst_square);
            if en_passant_capture {
                let en_passant_square = self.en_passant.unwrap();

                // calculate the square in which the en passant target is located
                let en_passant_capture_square = match self.active_color {
                    Color::White => (en_passant_square.0 + 1, en_passant_square.1).into(),
                    Color::Black => (en_passant_square.0 - 1, en_passant_square.1).into(),
                };

                self.set_piece(en_passant_capture_square, None);
            }

            let src_square_piece = self.get_piece(src_square);
            let dst_square_piece = self.get_piece(dst_square);

            // update castle rights before removing the source square piece
            self.update_castle_rights(r#move);

            // reset halfmove clock if a pawn is moved or a piece is captured
            if src_square_piece == Some(Piece::Pawn(self.active_color))
                || dst_square_piece.is_some()
                || en_passant_capture
            {
                self.halfmove_clock = 0;
            } else {
                self.halfmove_clock += 1;
            }

            if let Some(promotion_piece) = r#move.promotion {
                self.set_piece(dst_square, Some(promotion_piece));
            } else {
                self.set_piece(dst_square, src_square_piece);
            }

            self.set_piece(src_square, None);
        }

        self.active_color = self.active_color.invert();
        self.en_passant = r#move.en_passant;
        self.fullmove_number += match self.active_color {
            Color::White => 1,
            Color::Black => 0,
        };
    }

    /// Returns if a given move will leave the king in check.
    /// The move passed to this method should be legal and valid, otherwise undefined behavior may occur.
    pub(crate) fn future_check(&self, r#move: &Move) -> bool {
        let mut cloned_board = self.clone();
        cloned_board.make_move(r#move);
        cloned_board.active_color = cloned_board.active_color.invert();
        cloned_board.check()
    }

    /// Returns the pieces of the opposite active color that are attacking the given square.
    pub fn square_attackers(&self, src_square: Square) -> Vec<Piece> {
        let mut attacking_pieces = Vec::new();
        let color = self.active_color.invert();

        let pieces = [
            Piece::Pawn(color),
            Piece::Knight(color),
            Piece::Bishop(color),
            Piece::Rook(color),
            Piece::Queen(color),
            Piece::King(color),
        ];

        // starting from the square we are checking, iterate through all the directions
        // of each piece and check if there are any pieces attacking the square.
        for piece in pieces.iter() {
            for direction in piece.directions() {
                // pawns can only attack diagonally
                if piece == &Piece::Pawn(color) && direction.1 == 0 {
                    continue;
                }

                let mut src_square = match piece {
                    // since in this method we are going from the square we are checking to the source square,
                    // we need to invert the direction if the piece is a pawn.
                    Piece::Pawn(_) => Square(
                        (src_square.0 as i8 - direction.0) as usize,
                        (src_square.1 as i8 + direction.1) as usize,
                    ),
                    _ => Square(
                        (src_square.0 as i8 + direction.0) as usize,
                        (src_square.1 as i8 + direction.1) as usize,
                    ),
                };

                while (0..=7).contains(&src_square.0) && (0..=7).contains(&src_square.1) {
                    let src_square_piece = self.get_piece(src_square);
                    if src_square_piece.is_some_and(|p| &p != piece) {
                        break;
                    }

                    src_square += direction;

                    if src_square_piece.is_none() {
                        match piece {
                            Piece::Queen(_) => continue,
                            Piece::Rook(_) => continue,
                            Piece::Bishop(_) => continue,
                            Piece::Knight(_) => break,
                            Piece::King(_) => break,
                            Piece::Pawn(_) => break,
                        }
                    }

                    attacking_pieces.push(*piece);
                }
            }
        }

        attacking_pieces
    }

    /// Castles kingside for the given active color.
    /// This method assumes that the castle is legal.
    fn castle_kingside(&mut self) {
        let row = match self.active_color {
            Color::White => 7,
            Color::Black => 0,
        };

        let king_square = (row, 4).into();
        let rook_square = (row, 7).into();
        let new_king_square = (row, 6).into();
        let new_rook_square = (row, 5).into();

        self.set_piece(king_square, None);
        self.set_piece(rook_square, None);
        self.set_piece(new_king_square, Some(Piece::King(self.active_color)));
        self.set_piece(new_rook_square, Some(Piece::Rook(self.active_color)));
    }

    /// Castles queenside for the current active color.
    /// This method assumes that the castle is legal.
    fn castle_queenside(&mut self) {
        let row = match self.active_color {
            Color::White => 7,
            Color::Black => 0,
        };

        let king_square = (row, 4).into();
        let rook_square = (row, 0).into();
        let new_king_square = (row, 2).into();
        let new_rook_square = (row, 3).into();

        self.set_piece(king_square, None);
        self.set_piece(rook_square, None);
        self.set_piece(new_king_square, Some(Piece::King(self.active_color)));
        self.set_piece(new_rook_square, Some(Piece::Rook(self.active_color)));
    }

    /// Returns the square of the current active color king.
    fn king_square(&self) -> Square {
        for (row, col) in self.pieces.iter().enumerate() {
            for (col, piece) in col.iter().enumerate() {
                if piece == &Some(Piece::King(self.active_color)) {
                    return Square(row, col);
                }
            }
        }

        unreachable!("King can't be missing from the battle!")
    }

    /// Updates the castle rights given a move.
    fn update_castle_rights(&mut self, r#move: &Move) {
        // if the move is a castle, remove all castle rights for the current active color
        if r#move.castle.is_some() {
            match self.active_color {
                Color::White => self.castle_rights.retain(|x| {
                    x != &CastleRights::WhiteKingside && x != &CastleRights::WhiteQueenside
                }),
                Color::Black => self.castle_rights.retain(|x| {
                    x != &CastleRights::BlackKingside && x != &CastleRights::BlackQueenside
                }),
            }
        }

        // if the move is a king move, remove all castle rights for the current active color or
        // if the move is a rook move, remove the corresponding castle rights for the current active color
        if let Some(src_square) = r#move.src_square {
            let src_square_piece = self.get_piece(src_square);

            match src_square_piece {
                Some(Piece::King(Color::White)) => {
                    self.castle_rights.retain(|x| {
                        x != &CastleRights::WhiteKingside && x != &CastleRights::WhiteQueenside
                    });
                }

                Some(Piece::King(Color::Black)) => {
                    self.castle_rights.retain(|x| {
                        x != &CastleRights::BlackKingside && x != &CastleRights::BlackQueenside
                    });
                }

                Some(Piece::Rook(Color::White)) => {
                    if src_square == (7, 7) {
                        self.castle_rights
                            .retain(|x| x != &CastleRights::WhiteKingside);
                    } else if src_square == (7, 0) {
                        self.castle_rights
                            .retain(|x| x != &CastleRights::WhiteQueenside);
                    }
                }

                Some(Piece::Rook(Color::Black)) => {
                    if src_square == (0, 7) {
                        self.castle_rights
                            .retain(|x| x != &CastleRights::BlackKingside);
                    } else if src_square == (0, 0) {
                        self.castle_rights
                            .retain(|x| x != &CastleRights::BlackQueenside);
                    }
                }

                _ => {}
            }
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();

        for row in self.pieces.iter() {
            for piece in row.iter() {
                match piece {
                    Some(Piece::Pawn(Color::Black)) => board.push('♙'),
                    Some(Piece::Knight(Color::Black)) => board.push('♘'),
                    Some(Piece::Bishop(Color::Black)) => board.push('♗'),
                    Some(Piece::Rook(Color::Black)) => board.push('♖'),
                    Some(Piece::Queen(Color::Black)) => board.push('♕'),
                    Some(Piece::King(Color::Black)) => board.push('♔'),
                    Some(Piece::Pawn(Color::White)) => board.push('♟'),
                    Some(Piece::Knight(Color::White)) => board.push('♞'),
                    Some(Piece::Bishop(Color::White)) => board.push('♝'),
                    Some(Piece::Rook(Color::White)) => board.push('♜'),
                    Some(Piece::Queen(Color::White)) => board.push('♛'),
                    Some(Piece::King(Color::White)) => board.push('♚'),
                    None => board.push(' '),
                }
            }
            board.push('\n');
        }

        write!(f, "{}", board)
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}
