//! Chui Constants

/// The number of ranks on a standard chessboard.
pub const RANKS: u8 = 8;

/// The number of files on a standard chessboard.
pub const FILES: u8 = 8;

/// An array of chessboard files as usize format 8 elements long.
pub const INT_FILES: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

/// An array of chessboard files as &str format 8 elements long, representing alpha.
pub const STR_FILES: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

/// An array of chessboard files as &str format 8 elements long, representing numeric.
pub const STR_RANKS: [&str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];

/// An array of chessboard files as &str format 8 elements long, representing alpha.
pub const CHAR_FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

/// An array of chessboard files as &str format 8 elements long, representing numeric.
pub const CHAR_RANKS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

/// Compile in the version of this crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
