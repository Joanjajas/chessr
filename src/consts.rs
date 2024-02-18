pub const PIECE_TYPE_COUNT: usize = 6;
pub const TOTAL_SQUARES_COUNT: usize = 64;
pub const PLAYERS_COUNT: usize = 2;
pub const FEN_STARTING_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// Masks for each rank
pub const RANK_1: u64 = 0x00_00_00_00_00_00_00_FF;
pub const RANK_2: u64 = 0x00_00_00_00_00_00_FF_00;
pub const RANK_3: u64 = 0x00_00_00_00_00_FF_00_00;
pub const RANK_4: u64 = 0x00_00_00_00_FF_00_00_00;
pub const RANK_5: u64 = 0x00_00_00_FF_00_00_00_00;
pub const RANK_6: u64 = 0x00_00_FF_00_00_00_00_00;
pub const RANK_7: u64 = 0x00_FF_00_00_00_00_00_00;
pub const RANK_8: u64 = 0xFF_00_00_00_00_00_00_00;

// Masks for each file
pub const FILE_A: u64 = 0x80_80_80_80_80_80_80_80;
pub const FILE_B: u64 = 0x40_40_40_40_40_40_40_40;
pub const FILE_C: u64 = 0x20_20_20_20_20_20_20_20;
pub const FILE_D: u64 = 0x10_10_10_10_10_10_10_10;
pub const FILE_E: u64 = 0x08_08_08_08_08_08_08_08;
pub const FILE_F: u64 = 0x04_04_04_04_04_04_04_04;
pub const FILE_G: u64 = 0x02_02_02_02_02_02_02_02;
pub const FILE_H: u64 = 0x01_01_01_01_01_01_01_01;

// Masks for castling rights
pub const WHITE_KINGSIDE_CASTLE: u8 = 0b0001;
pub const WHITE_QUEENSIDE_CASTLE: u8 = 0b0010;
pub const BLACK_KINGSIDE_CASTLE: u8 = 0b0100;
pub const BLACK_QUEENSIDE_CASTLE: u8 = 0b1000;
pub const ALL_CASTLE_RIGHTS_WHITE: u8 = 0b0011;
pub const ALL_CASTLE_RIGHTS_BLACK: u8 = 0b1100;
pub const ALL_CASTLE_RIGHTS: u8 = 0b1111;
pub const NO_CASTLE_RIGHTS: u8 = 0b0000;
