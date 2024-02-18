use crate::castle::CastleRights;
use crate::color::Color;
use crate::consts::*;
use crate::fen::{self, FenParseError};
use crate::piece::Piece;
use crate::square::Square;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitBoard(pub u64);

impl std::ops::BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: BitBoard) {
        self.0 |= rhs.0;
    }
}

impl std::fmt::Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..8 {
            for j in 0..8 {
                let square = i * 8 + j;
                if self.0 & (1 << square) != 0 {
                    write!(f, "1")?;
                } else {
                    write!(f, "0")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    /// Array of pieces on the board.
    pub pieces_order: [Option<Piece>; TOTAL_SQUARES_COUNT],

    /// Bitboards for each piece type for both players.
    pub both_players_pieces: [BitBoard; PIECE_TYPE_COUNT],

    /// Bitboards for each player's pieces
    pub players_pieces: [BitBoard; PLAYERS_COUNT],

    /// Color of the players who moeves next.
    pub active_color: Color,

    /// Castling rights for both players.
    pub castle_rights: CastleRights,

    /// En passant target square.
    pub en_passant_target: Option<Square>,

    /// Number of moves since the last capture or pawn advance.
    pub halfmove_clock: u8,

    /// Number of completed turns in the game.
    pub fullmove_number: u8,
}

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
        fen::parse_fen(FEN_STARTING_POS).unwrap()
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
        fen::parse_fen(fen_str)
    }
}

impl std::default::Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fisrt_line = "┌───┬───┬───┬───┬───┬───┬───┬───┐";
        let last_line = "└───┴───┴───┴───┴───┴───┴───┴───┘";
        let horizontal_line = "├───┼───┼───┼───┼───┼───┼───┼───┤";
        let rows = ['8', '7', '6', '5', '4', '3', '2', '1'];

        writeln!(f, "{}", fisrt_line)?;

        for (i, row) in rows.iter().enumerate() {
            write!(f, "│")?;
            for (j, _) in (0..8).enumerate() {
                let square = i * 8 + j;
                match self.pieces_order[square] {
                    Some(piece) => write!(f, " {} ", piece)?,
                    None => write!(f, "   ")?,
                }
                write!(f, "│")?;
            }
            writeln!(f, " {}", row)?;
            if i < 7 {
                writeln!(f, "{}", horizontal_line)?;
            }
        }

        writeln!(f, "{}", last_line)?;
        writeln!(f, "  a   b   c   d   e   f   g   h")?;

        Ok(())
    }
}
