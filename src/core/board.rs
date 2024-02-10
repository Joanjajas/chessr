use std::collections::HashMap;

use crate::constants::{FEN_STARTING_POSITION, PAWN_CAPTURE_DIRECTIONS};
use crate::core::{movegen, CastleKind, CastleRights, Color, Move, Piece, SquareCoords};
use crate::fen::{self, FenParseError};

/// Represents a chess board.
///
/// The board is represented as an 8x8 array of [Piece]. Each piece is an
/// optional value, where `None` represents an empty square.
#[derive(Debug, Clone)]
pub struct Board {
    /// Board squares represented either by a [Piece] or `None` if the square
    /// is empty.
    pub squares: [[Option<Piece>; 8]; 8],

    /// Color of the player who moves next.
    pub active_color: Color,

    /// Castling availability for each player and castle type
    pub castle_rights: Vec<CastleRights>,

    /// En passant target square.
    pub en_passant_target: Option<SquareCoords>,

    /// Number of moves since the last capture or pawn advance.
    pub halfmove_clock: u32,

    /// Number of completed turns in the game.
    pub fullmove_number: u32,

    /// History of the board's positions.
    pub position_history: Vec<String>,
}

// TODO: PGN, replay games.
impl Board {
    /// Creates a new board with the starting position.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessr::Board;
    ///
    /// let board = Board::new();
    /// assert_eq!(
    ///     board.fen(),
    ///     "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    /// );
    /// ```
    pub fn new() -> Board {
        fen::fen_to_board(FEN_STARTING_POSITION).unwrap()
    }

    /// Creates a board from a FEN String.
    ///
    /// [Forsyth–Edwards Notation](https://www.chess.com/terms/fen-chess)
    /// (FEN) is a standard notation for describing a particular board position
    /// of a chess game.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessr::Board;
    ///
    /// pub const FEN_STARTING_POSITION: &str =
    ///     "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    ///
    /// let board = Board::from_fen(FEN_STARTING_POSITION).unwrap();
    /// assert_eq!(board.fen(), FEN_STARTING_POSITION);
    /// ```
    pub fn from_fen(fen_str: &str) -> Result<Board, FenParseError> {
        fen::fen_to_board(fen_str)
    }

    /// Creates a FEN Utring representation of the current the board.
    ///
    /// [Forsyth–Edwards Notation](https://www.chess.com/terms/fen-chess)
    /// (FEN) is a standard notation for describing a particular board position
    /// of a chess game.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessr::Board;
    ///
    /// let board = Board::new();
    /// assert_eq!(
    ///     board.fen(),
    ///     "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    /// );
    /// ```
    pub fn fen(&self) -> String {
        fen::board_to_fen(self)
    }

    /// Returns a vector of all the pieces and their respective squares that
    /// are checking the king in the current position.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessr::Board;
    ///
    /// let board = Board::from_fen("rnbqk1nr/ppp2ppp/4p3/3p4/1bPP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 2 4").unwrap();
    /// assert_eq!(board.checkers().len(), 1);
    /// assert_eq!(board.checkers()[0].0.to_fen_char(), 'b');
    /// assert_eq!(board.checkers()[0].1.to_string(), "b4");
    pub fn checkers(&self) -> Vec<(Piece, SquareCoords)> {
        self.square_attackers(self.king_square())
    }

    /// Returns true if there is a check in the current position.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessr::Board;
    ///
    /// let board = Board::from_fen("rnbqk1nr/ppp2ppp/4p3/3p4/1bPP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 2 4")
    ///     .unwrap();
    /// assert_eq!(board.check(), true);
    /// ```
    pub fn check(&self) -> bool {
        !self.checkers().is_empty()
    }

    /// Returns true if there is a checkmate in the current position.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessr::Board;
    ///
    /// let board =
    ///     Board::from_fen("rnb1kbnr/pppp1ppp/4p3/8/5PPq/8/PPPPP2P/RNBQKBNR w KQkq - 1 3").unwrap();
    /// assert_eq!(board.checkmate(), true);
    /// ```
    pub fn checkmate(&self) -> bool {
        self.check() && self.legal_moves().is_empty()
    }

    /// Returns true if there is a stalemate in the current position.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessr::Board;
    ///
    /// let board = Board::from_fen("8/8/8/8/8/2k5/2p5/2K5 w - - 0 1").unwrap();
    /// assert_eq!(board.stalemate(), true);
    /// ```
    pub fn stalemate(&self) -> bool {
        !self.check() && self.legal_moves().is_empty()
    }

    /// Returns true if 50 moves have been made without a pawn move or a
    /// capture.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessr::Board;
    ///
    /// let board = Board::new();
    /// assert_eq!(board.fifty_move_rule(), false);
    /// ```
    pub fn fifty_move_rule(&self) -> bool {
        self.halfmove_clock >= 50
    }

    /// Returns true if the current position is a draw by threefold repetition.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessr::Board;
    ///
    /// let mut board = Board::new();
    ///
    /// for r#move in &[
    ///     "e4", "e5", "Nf3", "Nf6", "Ng1", "Ng8", "Nf3", "Nf6", "Ng1", "Ng8",
    /// ] {
    ///     board.make_move(r#move);
    /// }
    ///
    /// assert_eq!(board.threefold_repetition(), true);
    /// ```
    pub fn threefold_repetition(&self) -> bool {
        let mut hash_map = HashMap::new();

        for pos in &self.position_history {
            let pos: String = pos.split_whitespace().take(4).collect();
            *hash_map.entry(pos).or_insert(0) += 1;
        }

        hash_map.iter().any(|(_, &count)| count >= 3)
    }

    /// Returns true if the current position is a draw by insufficient material.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessr::Board;
    ///
    /// let board = Board::from_fen("2k5/4b3/8/8/8/8/8/2K1B1B1 w - - 0 1").unwrap();
    /// assert_eq!(board.insufficient_material(), true);
    /// ```
    pub fn insufficient_material(&self) -> bool {
        let mut piece_count = 0;
        let mut knights = Vec::new();
        let mut bishops = Vec::new();

        let mut row_offset = 0;
        for (i, &piece) in self.squares.iter().flatten().enumerate() {
            // the color for the first square in the row alternates for each row,
            // so we need to set an offset every time we reach the end of a row.
            if i % 8 == 0 && i != 0 {
                row_offset += 1;
            }

            if let Some(piece) = piece {
                match piece {
                    Piece::Bishop(_) => {
                        // because we need to know the color of the square in
                        // which the bishops are, instead of pushing a piece
                        // into the vector, we push the color of the square.
                        let color = match (i + row_offset) % 2 {
                            0 => Color::White,
                            _ => Color::Black,
                        };
                        bishops.push(color)
                    }
                    Piece::Knight(_) => knights.push(piece),
                    _ => (),
                }

                piece_count += 1;
            }
        }

        // king vs king
        if piece_count == 2 {
            return true;
        }

        // king and bishop vs king or king and knight vs king
        if piece_count == 3 && (bishops.len() == 1 || knights.len() == 1) {
            return true;
        }

        // king and bishop vs king and bishop with the bishops on the same color
        // or king and any number of bishops vs king and any number of bishops
        // in the same color
        if piece_count == bishops.len() + 2 && bishops.windows(2).all(|c| c[0] == c[1]) {
            return true;
        }

        false
    }

    /// Returns true if the current position is a draw.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessr::Board;
    ///
    /// let board = Board::from_fen("8/8/1k6/5K2/8/8/4N3/8 b - - 0 2").unwrap();
    /// assert_eq!(board.draw(), true);
    pub fn draw(&self) -> bool {
        self.stalemate()
            || self.insufficient_material()
            || self.fifty_move_rule()
            || self.threefold_repetition()
    }

    /// Makes a move on the board given its notation in [UCI](https://en.wikipedia.org/wiki/Universal_Chess_Interface)
    /// protocol format notation. This method will accpedt either moves with
    /// source and destination squares separated by a '-' or moves with source
    /// and destination squares putted all together. Both "e2e4" and "e2-e4"
    /// will be considered valid.
    ///
    /// If the move notation is invalid or the move is not legal, no move will
    /// be applied. Also returns the move applied to the board.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessr::Board;
    ///
    /// let mut board = Board::new();
    /// let r#move = board.make_uci_move("e2e4");
    ///
    /// assert!(r#move.is_some());
    /// assert_eq!(
    ///     board.fen(),
    ///     "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
    /// );
    /// ```
    pub fn make_uci_move(&mut self, uci_str: &str) -> Option<Move> {
        let r#move = Move::from_uci(uci_str, self);

        if let Some(ref r#move) = r#move {
            if self.legal_moves().contains(r#move) {
                self.apply_move(r#move);
            }
        }

        r#move
    }

    /// Makes a move on the board given its [algebraic notation](https://www.chess.com/terms/chess-notation).
    /// If the move notation is invalid or the move is not legal, no move will
    /// be applied. Also returns the move that was applied.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessr::Board;
    ///
    /// let mut board = Board::new();
    /// let r#move = board.make_san_move("e4");
    ///
    /// assert!(r#move.is_some());
    /// assert_eq!(
    ///     board.fen(),
    ///     "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
    /// );
    /// ```
    pub fn make_san_move(&mut self, algebraic_str: &str) -> Option<Move> {
        let r#move = Move::from_san(algebraic_str, self);

        if let Some(ref r#move) = r#move {
            if self.legal_moves().contains(r#move) {
                self.apply_move(r#move);
            }
        }

        r#move
    }

    /// Tries to make a move, accepting both standard and non-standard algebraic
    /// notation. For making UCI moves or SAN moves see
    /// [make_uci_move()](crate::Board::make_uci_move())
    /// and [make_san_move()](crate::Board::make_san_move())
    /// functions.
    ///
    /// # Examples
    /// ```
    /// use chessr::Board;
    ///
    /// let mut board = Board::new();
    ///
    /// // Standard algebraic notation.
    /// let r#move = board.make_move("e4");
    /// assert_eq!(r#move.is_some(), true);
    ///
    /// // Long algebraic notation without '-'.
    /// let r#move = board.make_move("e7e5");
    /// assert_eq!(r#move.is_some(), true);
    ///
    /// // Long algebraic notation with '-'.
    /// let r#move = board.make_move("f1-c4");
    /// assert_eq!(r#move.is_some(), true);
    /// ```
    pub fn make_move(&mut self, move_str: &str) -> Option<Move> {
        // try to parse the move as UCI.
        if let Some(r#move) = Move::from_uci(move_str, self) {
            if self.legal_moves().contains(&r#move) {
                self.apply_move(&r#move);
                return Some(r#move);
            }
        }

        // try to parse the move as SAN.
        if let Some(r#move) = Move::from_san(move_str, self) {
            if self.legal_moves().contains(&r#move) {
                self.apply_move(&r#move);
                return Some(r#move);
            }
        }

        None
    }

    /// Returns a vec of [Move] containing all possible legal moves in the
    /// current position.
    ///
    /// # Examples
    ///
    /// ```
    /// use chessr::Board;
    ///
    /// let mut board = Board::new();
    /// assert_eq!(board.legal_moves().len(), 20);
    /// ```
    pub fn legal_moves(&self) -> Vec<Move> {
        movegen::generate_legal_moves(self)
    }

    /// Returns the piece located at the given square, if any. If the square
    /// provided is out of bounds, the method will panic.
    pub(crate) fn get_piece(&self, square_coords: SquareCoords) -> Option<Piece> {
        self.squares[square_coords.0][square_coords.1]
    }

    /// Sets the piece at the given square. To remove a piece from a square,
    /// pass `None` as the piece. If the square provided is out of bounds, the
    /// method will panic.
    pub(crate) fn set_piece(&mut self, square_coords: SquareCoords, piece: Option<Piece>) {
        self.squares[square_coords.0][square_coords.1] = piece;
    }

    /// Applies a move on the board, updating the board state.
    /// This method assumes that the move is legal and valid, otherwise
    /// undefined behavior may occur.
    pub(crate) fn apply_move(&mut self, r#move: &Move) {
        // handle castling
        if let Some(ref castle) = r#move.castle {
            match castle {
                CastleKind::Kingside => self.castle_kingside(),
                CastleKind::Queenside => self.castle_queenside(),
            }
        }

        // handle normal move and en passant
        if let (Some(src_square), Some(dst_square)) = (r#move.src_square, r#move.dst_square) {
            // handle en pasant capture
            if self.en_passant_target.is_some_and(|s| s == dst_square) {
                let en_passant_square = self.en_passant_target.unwrap();

                // calculate the square in which the en passant target is located
                let en_passant_capture_square = match self.active_color {
                    Color::White => (en_passant_square.0 + 1, en_passant_square.1).into(),
                    Color::Black => (en_passant_square.0 - 1, en_passant_square.1).into(),
                };

                self.set_piece(en_passant_capture_square, None);
            }

            // reset halfmove clock if a pawn is moved or a piece is captured
            if r#move.piece == Some(Piece::Pawn(self.active_color)) || r#move.capture {
                self.halfmove_clock = 0;
            } else {
                self.halfmove_clock += 1;
            }

            // handle promotion
            if let Some(promotion_piece) = r#move.promotion {
                self.set_piece(dst_square, Some(promotion_piece));
            } else {
                self.set_piece(dst_square, r#move.piece);
            }

            self.set_piece(src_square, None);
        }

        self.update_castle_rights(r#move);
        self.position_history.push(self.fen());
        self.active_color = self.active_color.invert();
        self.en_passant_target = self.update_en_passant_target_square(r#move);
        self.fullmove_number += match self.active_color {
            Color::White => 1,
            Color::Black => 0,
        };
    }

    /// Returns if a given move will leave the king in check.
    /// The move passed to this method is assumed to be legal and valid,
    /// otherwise undefined behavior may occur.
    pub(crate) fn future_check(&self, r#move: &Move) -> bool {
        let mut cloned_board = self.clone();
        cloned_board.apply_move(r#move);
        cloned_board.active_color = cloned_board.active_color.invert();
        cloned_board.check()
    }

    /// Returns the pieces an its respectives squares from where a given square is being attacked.
    pub(crate) fn square_attackers(&self, src_square: SquareCoords) -> Vec<(Piece, SquareCoords)> {
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
        for piece in &pieces {
            for direction in &piece.directions() {
                // pawns can only attack diagonally
                if piece == &Piece::Pawn(color) && direction.1 == 0 {
                    continue;
                }

                let mut src_square = match piece {
                    // since in this method we are going from the square we are checking to the
                    // source square, we need to invert the direction if the
                    // piece is a pawn.
                    Piece::Pawn(_) => SquareCoords(
                        (src_square.0 as i8 - direction.0) as usize,
                        (src_square.1 as i8 + direction.1) as usize,
                    ),
                    _ => SquareCoords(
                        (src_square.0 as i8 + direction.0) as usize,
                        (src_square.1 as i8 + direction.1) as usize,
                    ),
                };

                while (0..=7).contains(&src_square.0) && (0..=7).contains(&src_square.1) {
                    let src_square_piece = self.get_piece(src_square);
                    if src_square_piece.is_some_and(|p| &p != piece) {
                        break;
                    }

                    if src_square_piece.is_some() {
                        attacking_pieces.push((*piece, src_square));
                    }

                    src_square += direction;
                    match piece {
                        Piece::Queen(_) => continue,
                        Piece::Rook(_) => continue,
                        Piece::Bishop(_) => continue,
                        Piece::Knight(_) => break,
                        Piece::King(_) => break,
                        Piece::Pawn(_) => break,
                    }
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

    /// Checks if en passant is possible in next turn given a move.
    fn update_en_passant_target_square(&self, r#move: &Move) -> Option<SquareCoords> {
        if let (Some(src_square), Some(dst_square)) = (r#move.src_square, r#move.dst_square) {
            // if the move is not a double pawn move, return false
            if r#move.piece != Some(Piece::Pawn(self.active_color))
                || (dst_square.0 as i8 - src_square.0 as i8).abs() != 2
            {
                return None;
            }

            let en_passant_target: SquareCoords = {
                match self.active_color {
                    Color::Black => (dst_square.0 - 1, dst_square.1).into(),
                    Color::White => (dst_square.0 + 1, dst_square.1).into(),
                }
            };

            for direction in &PAWN_CAPTURE_DIRECTIONS {
                let src_square = en_passant_target + direction;

                if !(0..=7).contains(&src_square.0) || !(0..=7).contains(&src_square.1) {
                    continue;
                }

                if self.get_piece(src_square) == Some(Piece::Pawn(self.active_color.invert())) {
                    return Some(en_passant_target);
                }
            }
        }

        None
    }

    /// Returns the square of the current active color king.
    fn king_square(&self) -> SquareCoords {
        for (row, &col) in self.squares.iter().enumerate() {
            for (col, &piece) in col.iter().enumerate() {
                if piece == Some(Piece::King(self.active_color)) {
                    return SquareCoords(row, col);
                }
            }
        }

        unreachable!("King can't be missing from the battle!")
    }

    /// Updates the castle rights given a move.
    fn update_castle_rights(&mut self, r#move: &Move) {
        // castling move
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

        // white king moves
        if r#move.piece.is_some_and(|p| p == Piece::King(Color::White)) {
            self.castle_rights.retain(|x| {
                x != &CastleRights::WhiteKingside && x != &CastleRights::WhiteQueenside
            });
        }

        // black king moves
        if r#move.piece.is_some_and(|p| p == Piece::King(Color::Black)) {
            self.castle_rights.retain(|x| {
                x != &CastleRights::BlackKingside && x != &CastleRights::BlackQueenside
            });
        }

        // white kingside rook moves or is captured
        if r#move.src_square.is_some_and(|s| s == (7, 7))
            || r#move.dst_square.is_some_and(|s| s == (7, 7))
        {
            self.castle_rights
                .retain(|x| x != &CastleRights::WhiteKingside);
        }

        // white queenside rook moves or is captured
        if r#move.src_square.is_some_and(|s| s == (7, 0))
            || r#move.dst_square.is_some_and(|s| s == (7, 0))
        {
            self.castle_rights
                .retain(|x| x != &CastleRights::WhiteQueenside);
        }

        // black kingside rook moves or is captured
        if r#move.src_square.is_some_and(|s| s == (0, 7))
            || r#move.dst_square.is_some_and(|s| s == (0, 7))
        {
            self.castle_rights
                .retain(|x| x != &CastleRights::BlackKingside);
        }

        // black queenside rook moves or is captured
        if r#move.src_square.is_some_and(|s| s == (0, 0))
            || r#move.dst_square.is_some_and(|s| s == (0, 0))
        {
            self.castle_rights
                .retain(|x| x != &CastleRights::BlackQueenside);
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fisrt_line = "┌───┬───┬───┬───┬───┬───┬───┬───┐";
        let last_line = "└───┴───┴───┴───┴───┴───┴───┴───┘";
        let horizontal_line = "├───┼───┼───┼───┼───┼───┼───┼───┤";
        let rows = ['8', '7', '6', '5', '4', '3', '2', '1'];
        let cols = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

        writeln!(f, "{}", fisrt_line)?;

        for (i, &row) in self.squares.iter().enumerate() {
            write!(f, "│")?;
            for (j, &piece) in row.iter().enumerate() {
                if j == 7 {
                    match piece {
                        Some(piece) => write!(f, " {} │ {}", piece, rows[i]),
                        None => write!(f, "   │ {}", rows[i]),
                    }?;
                } else {
                    match piece {
                        Some(piece) => write!(f, " {} │", piece),
                        None => write!(f, "   │"),
                    }?;
                }
            }

            if i != 7 {
                writeln!(f, "\n{}", horizontal_line)?;
            } else {
                writeln!(f, "\n{}", last_line)?;
            }
        }

        for col in &cols {
            write!(f, "  {} ", col)?;
        }

        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}
