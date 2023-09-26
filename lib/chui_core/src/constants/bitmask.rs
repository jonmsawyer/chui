//! Chui Constants

#![allow(dead_code)]

//
// Bitmasks
//

/// File A bitmask.
pub const FILE_A: u64 = 0x101010101010101;
/// File B bitmask.
pub const FILE_B: u64 = 0x202020202020202;
/// File C bitmask.
pub const FILE_C: u64 = 0x404040404040404;
/// File D bitmask.
pub const FILE_D: u64 = 0x808080808080808;
/// File E bitmask.
pub const FILE_E: u64 = 0x1010101010101010;
/// File F bitmask.
pub const FILE_F: u64 = 0x2020202020202020;
/// File G bitmask.
pub const FILE_G: u64 = 0x4040404040404040;
/// File H bitmask.
pub const FILE_H: u64 = 0x8080808080808080;
/// Files bitmasks.
pub const FILES: [u64; 8] = [
    FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H,
];

/// Rank 1 bitmask.
pub const RANK_1: u64 = 0xFF;
/// Rank 2 bitmask.
pub const RANK_2: u64 = 0xFF00;
/// Rank 3 bitmask.
pub const RANK_3: u64 = 0xFF0000;
/// Rank 4 bitmask.
pub const RANK_4: u64 = 0xFF000000;
/// Rank 5 bitmask.
pub const RANK_5: u64 = 0xFF00000000;
/// Rank 6 bitmask.
pub const RANK_6: u64 = 0xFF0000000000;
/// Rank 7 bitmask.
pub const RANK_7: u64 = 0xFF000000000000;
/// Rank 8 bitmask.
pub const RANK_8: u64 = 0xFF00000000000000;
/// Ranks bitmasks.
pub const RANKS: [u64; 8] = [
    RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8,
];

/// Light squares bitmask.
pub const LIGHT_SQUARES: u64 = 0x55AA55AA55AA55AA;
/// Dark squares bitmask.
pub const DARK_SQUARES: u64 = 0xAA55AA55AA55AA55;

/// a1 bitmask.
pub const A1: u64 = FILE_A & RANK_1;
/// a2 bitmask.
pub const A2: u64 = FILE_A & RANK_2;
/// a3 bitmask.
pub const A3: u64 = FILE_A & RANK_3;
/// a4 bitmask.
pub const A4: u64 = FILE_A & RANK_4;
/// a5 bitmask.
pub const A5: u64 = FILE_A & RANK_5;
/// a6 bitmask.
pub const A6: u64 = FILE_A & RANK_6;
/// a7 bitmask.
pub const A7: u64 = FILE_A & RANK_7;
/// a8 bitmask.
pub const A8: u64 = FILE_A & RANK_8;

/// b1 bitmask.
pub const B1: u64 = FILE_B & RANK_1;
/// b2 bitmask.
pub const B2: u64 = FILE_B & RANK_2;
/// b3 bitmask.
pub const B3: u64 = FILE_B & RANK_3;
/// b4 bitmask.
pub const B4: u64 = FILE_B & RANK_4;
/// b5 bitmask.
pub const B5: u64 = FILE_B & RANK_5;
/// b6 bitmask.
pub const B6: u64 = FILE_B & RANK_6;
/// b7 bitmask.
pub const B7: u64 = FILE_B & RANK_7;
/// b8 bitmask.
pub const B8: u64 = FILE_B & RANK_8;

/// c1 bitmask.
pub const C1: u64 = FILE_C & RANK_1;
/// c2 bitmask.
pub const C2: u64 = FILE_C & RANK_2;
/// c3 bitmask.
pub const C3: u64 = FILE_C & RANK_3;
/// c4 bitmask.
pub const C4: u64 = FILE_C & RANK_4;
/// c5 bitmask.
pub const C5: u64 = FILE_C & RANK_5;
/// c6 bitmask.
pub const C6: u64 = FILE_C & RANK_6;
/// c7 bitmask.
pub const C7: u64 = FILE_C & RANK_7;
/// c8 bitmask.
pub const C8: u64 = FILE_C & RANK_8;

/// d1 bitmask.
pub const D1: u64 = FILE_D & RANK_1;
/// d2 bitmask.
pub const D2: u64 = FILE_D & RANK_2;
/// d3 bitmask.
pub const D3: u64 = FILE_D & RANK_3;
/// d4 bitmask.
pub const D4: u64 = FILE_D & RANK_4;
/// d5 bitmask.
pub const D5: u64 = FILE_D & RANK_5;
/// d6 bitmask.
pub const D6: u64 = FILE_D & RANK_6;
/// d7 bitmask.
pub const D7: u64 = FILE_D & RANK_7;
/// d8 bitmask.
pub const D8: u64 = FILE_D & RANK_8;

/// e1 bitmask.
pub const E1: u64 = FILE_E & RANK_1;
/// e2 bitmask.
pub const E2: u64 = FILE_E & RANK_2;
/// e3 bitmask.
pub const E3: u64 = FILE_E & RANK_3;
/// e4 bitmask.
pub const E4: u64 = FILE_E & RANK_4;
/// e5 bitmask.
pub const E5: u64 = FILE_E & RANK_5;
/// e6 bitmask.
pub const E6: u64 = FILE_E & RANK_6;
/// e7 bitmask.
pub const E7: u64 = FILE_E & RANK_7;
/// e8 bitmask.
pub const E8: u64 = FILE_E & RANK_8;

/// f1 bitmask.
pub const F1: u64 = FILE_F & RANK_1;
/// f2 bitmask.
pub const F2: u64 = FILE_F & RANK_2;
/// f3 bitmask.
pub const F3: u64 = FILE_F & RANK_3;
/// f4 bitmask.
pub const F4: u64 = FILE_F & RANK_4;
/// f5 bitmask.
pub const F5: u64 = FILE_F & RANK_5;
/// f6 bitmask.
pub const F6: u64 = FILE_F & RANK_6;
/// f7 bitmask.
pub const F7: u64 = FILE_F & RANK_7;
/// f8 bitmask.
pub const F8: u64 = FILE_F & RANK_8;

/// g1 bitmask.
pub const G1: u64 = FILE_G & RANK_1;
/// g2 bitmask.
pub const G2: u64 = FILE_G & RANK_2;
/// g3 bitmask.
pub const G3: u64 = FILE_G & RANK_3;
/// g4 bitmask.
pub const G4: u64 = FILE_G & RANK_4;
/// g5 bitmask.
pub const G5: u64 = FILE_G & RANK_5;
/// g6 bitmask.
pub const G6: u64 = FILE_G & RANK_6;
/// g7 bitmask.
pub const G7: u64 = FILE_G & RANK_7;
/// g8 bitmask.
pub const G8: u64 = FILE_G & RANK_8;

/// h1 bitmask.
pub const H1: u64 = FILE_H & RANK_1;
/// h2 bitmask.
pub const H2: u64 = FILE_H & RANK_2;
/// h3 bitmask.
pub const H3: u64 = FILE_H & RANK_3;
/// h4 bitmask.
pub const H4: u64 = FILE_H & RANK_4;
/// h5 bitmask.
pub const H5: u64 = FILE_H & RANK_5;
/// h6 bitmask.
pub const H6: u64 = FILE_H & RANK_6;
/// h7 bitmask.
pub const H7: u64 = FILE_H & RANK_7;
/// h8 bitmask.
pub const H8: u64 = FILE_H & RANK_8;
