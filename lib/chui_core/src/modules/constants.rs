//! Chui Constants

#![allow(dead_code)]

/// The number of ranks on a standard chessboard.
pub const RANKS: u8 = 8;

/// The number of files on a standard chessboard.
pub const FILES: u8 = 8;

/// An array of chessboard files as usize format 8 elements long.
pub const INT_FILES: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

/// An array of chessboard files as &str format 8 elements long, representing alpha.
pub const STR_FILES: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

/// An array of chessboard ranks as &str format 8 elements long, representing numeric.
pub const STR_RANKS: [&str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];

/// An array of chessboard files as &str format 8 elements long, representing alpha.
pub const CHAR_FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

/// An array of chessboard ranks as &str format 8 elements long, representing numeric.
pub const CHAR_RANKS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

/// Compile in the version of this crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

//
// A coordinates
//

/// (char, u8) coordinate for a1
pub const A1: (char, u8) = ('a', 1);
/// (char, u8) coordinate for a2
pub const A2: (char, u8) = ('a', 2);
/// (char, u8) coordinate for a3
pub const A3: (char, u8) = ('a', 3);
/// (char, u8) coordinate for a4
pub const A4: (char, u8) = ('a', 4);
/// (char, u8) coordinate for a5
pub const A5: (char, u8) = ('a', 5);
/// (char, u8) coordinate for a6
pub const A6: (char, u8) = ('a', 6);
/// (char, u8) coordinate for a7
pub const A7: (char, u8) = ('a', 7);
/// (char, u8) coordinate for a8
pub const A8: (char, u8) = ('a', 8);

//
// B coordinates
//

/// (char, u8) coordinate for b1
pub const B1: (char, u8) = ('b', 1);
/// (char, u8) coordinate for b2
pub const B2: (char, u8) = ('b', 2);
/// (char, u8) coordinate for b3
pub const B3: (char, u8) = ('b', 3);
/// (char, u8) coordinate for b4
pub const B4: (char, u8) = ('b', 4);
/// (char, u8) coordinate for b5
pub const B5: (char, u8) = ('b', 5);
/// (char, u8) coordinate for b6
pub const B6: (char, u8) = ('b', 6);
/// (char, u8) coordinate for b7
pub const B7: (char, u8) = ('b', 7);
/// (char, u8) coordinate for b8
pub const B8: (char, u8) = ('b', 8);

//
// C coordinates
//

/// (char, u8) coordinate for c1
pub const C1: (char, u8) = ('c', 1);
/// (char, u8) coordinate for c2
pub const C2: (char, u8) = ('c', 2);
/// (char, u8) coordinate for c3
pub const C3: (char, u8) = ('c', 3);
/// (char, u8) coordinate for c4
pub const C4: (char, u8) = ('c', 4);
/// (char, u8) coordinate for c5
pub const C5: (char, u8) = ('c', 5);
/// (char, u8) coordinate for c6
pub const C6: (char, u8) = ('c', 6);
/// (char, u8) coordinate for c7
pub const C7: (char, u8) = ('c', 7);
/// (char, u8) coordinate for c8
pub const C8: (char, u8) = ('c', 8);

//
// D coordinates
//

/// (char, u8) coordinate for d1
pub const D1: (char, u8) = ('d', 1);
/// (char, u8) coordinate for d2
pub const D2: (char, u8) = ('d', 2);
/// (char, u8) coordinate for d3
pub const D3: (char, u8) = ('d', 3);
/// (char, u8) coordinate for d4
pub const D4: (char, u8) = ('d', 4);
/// (char, u8) coordinate for d5
pub const D5: (char, u8) = ('d', 5);
/// (char, u8) coordinate for d6
pub const D6: (char, u8) = ('d', 6);
/// (char, u8) coordinate for d7
pub const D7: (char, u8) = ('d', 7);
/// (char, u8) coordinate for d8
pub const D8: (char, u8) = ('d', 8);

//
// E coordinates
//

/// (char, u8) coordinate for e1
pub const E1: (char, u8) = ('e', 1);
/// (char, u8) coordinate for e2
pub const E2: (char, u8) = ('e', 2);
/// (char, u8) coordinate for e3
pub const E3: (char, u8) = ('e', 3);
/// (char, u8) coordinate for e4
pub const E4: (char, u8) = ('e', 4);
/// (char, u8) coordinate for e5
pub const E5: (char, u8) = ('e', 5);
/// (char, u8) coordinate for e6
pub const E6: (char, u8) = ('e', 6);
/// (char, u8) coordinate for e7
pub const E7: (char, u8) = ('e', 7);
/// (char, u8) coordinate for e8
pub const E8: (char, u8) = ('e', 8);

//
// F coordinates
//

/// (char, u8) coordinate for f1
pub const F1: (char, u8) = ('f', 1);
/// (char, u8) coordinate for f2
pub const F2: (char, u8) = ('f', 2);
/// (char, u8) coordinate for f3
pub const F3: (char, u8) = ('f', 3);
/// (char, u8) coordinate for f4
pub const F4: (char, u8) = ('f', 4);
/// (char, u8) coordinate for f5
pub const F5: (char, u8) = ('f', 5);
/// (char, u8) coordinate for f6
pub const F6: (char, u8) = ('f', 6);
/// (char, u8) coordinate for f7
pub const F7: (char, u8) = ('f', 7);
/// (char, u8) coordinate for f8
pub const F8: (char, u8) = ('f', 8);

//
// G coordinates
//

/// (char, u8) coordinate for g1
pub const G1: (char, u8) = ('g', 1);
/// (char, u8) coordinate for g2
pub const G2: (char, u8) = ('g', 2);
/// (char, u8) coordinate for g3
pub const G3: (char, u8) = ('g', 3);
/// (char, u8) coordinate for g4
pub const G4: (char, u8) = ('g', 4);
/// (char, u8) coordinate for g5
pub const G5: (char, u8) = ('g', 5);
/// (char, u8) coordinate for g6
pub const G6: (char, u8) = ('g', 6);
/// (char, u8) coordinate for g7
pub const G7: (char, u8) = ('g', 7);
/// (char, u8) coordinate for g8
pub const G8: (char, u8) = ('g', 8);

//
// H coordinates
//

/// (char, u8) coordinate for h1
pub const H1: (char, u8) = ('h', 1);
/// (char, u8) coordinate for h2
pub const H2: (char, u8) = ('h', 2);
/// (char, u8) coordinate for h3
pub const H3: (char, u8) = ('h', 3);
/// (char, u8) coordinate for h4
pub const H4: (char, u8) = ('h', 4);
/// (char, u8) coordinate for h5
pub const H5: (char, u8) = ('h', 5);
/// (char, u8) coordinate for h6
pub const H6: (char, u8) = ('h', 6);
/// (char, u8) coordinate for h7
pub const H7: (char, u8) = ('h', 7);
/// (char, u8) coordinate for h8
pub const H8: (char, u8) = ('h', 8);
