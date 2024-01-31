#![cfg_attr(rustfmt, rustfmt_skip)]
pub const FEN_STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const CASTLE_REGEX: &str = r"^(O-O|O-O-O|0-0|0-0-0|o-o|o-o-o)(\+|\#)?$";
pub const PAWN_MOVE_REGEX: &str = r"^([a-h])([2-7])(\+|\#)?$";
pub const PIECE_MOVE_REGEX: &str = r"^([KQBNR])([a-h])([1-8])(\+|\#)?$";
pub const PAWN_CAPTURE_REGEX: &str = r"^([a-h])x([a-h])([2-7])(\+|\#)?$";
pub const PIECE_CAPTURE_REGEX: &str = r"^([KQBNR])x([a-h])([1-8])(\+|\#)?$";
pub const PAWN_PROMOTION_REGEX: &str = r"^([a-h])(1|8)=([QBNR])(\+|\#)?$";
pub const PAWN_CAPTURE_PROMOTION_REGEX: &str = r"^([a-h])x([a-h])(1|8)=([QBNR])(\+|\#)?$";
pub const PIECE_MOVE_ROW_DISAMBIGUATION_REGEX: &str = r"^([KQBNR])([1-8])([a-h])([1-8])(\+|\#)?$";
pub const PIECE_MOVE_COLUMN_DISAMBIGUATION_REGEX: &str = r"^([KQBNR])([a-h])([a-h])([1-8])(\+|\#)?$";
pub const PIECE_MOVE_ROW_AND_COLUMN_DISAMBIGUATION_REGEX: &str = r"^([KQBNR])([a-h])([1-8])([a-h])([1-8])(\+|\#)?$";
pub const PIECE_CAPTURE_ROW_DISAMBIGUATION_REGEX: &str = r"^([KQBNR])([1-8])x([a-h])([1-8])(\+|\#)?$";
pub const PIECE_CAPTURE_COLUMN_DISAMBIGUATION_REGEX: &str = r"^([KQBNR])([a-h])x([a-h])([1-8])(\+|\#)?$";
pub const PIECE_CAPTURE_ROW_AND_COLUMN_DISAMBIGUATION_REGEX: &str = r"^([KQBNR])([a-h])([1-8])x([a-h])([1-8])(\+|\#)?$";

pub const PAWN_MOVE_DIRECTIONS: [(i8, i8); 2] = [(1, 0), (2, 0)];
pub const PAWN_CAPTURE_DIRECTIONS: [(i8, i8); 2] = [(1, 1), (1, -1)];
pub const ROOK_DIRECTIONS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
pub const BISHOP_DIRECTIONS: [(i8, i8); 4] = [(1, 1), (-1, 1), (-1, -1), (1, -1)];
pub const KNIGHT_DIRECTIONS: [(i8, i8); 8] = [ (2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2), ];
pub const KING_DIRECTIONS: [(i8, i8); 8] = [ (1, 0), (1, 1), (1, -1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), ];
pub const QUEEN_DIRECTIONS: [(i8, i8); 8] = [ (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1), ];

