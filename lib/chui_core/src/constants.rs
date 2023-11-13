//! Chui Constants

#![allow(dead_code)]

/// The number of ranks on a standard chessboard.
pub const RANKS: u8 = 8;

/// The number of files on a standard chessboard.
pub const FILES: u8 = 8;

/// An array of chessboard files as usize format 8 elements long.
pub const INT_FILES: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

/// An array of chessboard ranks as usize format 8 elements long.
pub const INT_RANKS: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

/// An array of chessboard files as &str format 8 elements long, representing alpha.
pub const STR_FILES: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

/// An array of chessboard ranks as &str format 8 elements long, representing numeric.
pub const STR_RANKS: [&str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];

/// An array of chessboard files as &str format 8 elements long, representing alpha.
pub const CHAR_FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

/// An array of chessboard ranks as &str format 8 elements long, representing numeric.
pub const CHAR_RANKS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

/// An array of chessboard pieces in &str format 5 elements long.
pub const STR_PROMOTION_PIECES: [&str; 8] = ["Q", "q", "R", "r", "B", "b", "N", "n"];

/// An array of chessboard pieces in &str format 5 elements long.
pub const CHAR_PROMOTION_PIECES: [char; 8] = ['Q', 'q', 'R', 'r', 'B', 'b', 'N', 'n'];

/// Compile in the version of this crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod bitmask;
pub mod coord;
