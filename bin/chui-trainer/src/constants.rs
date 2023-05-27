//! Chui Coordinate Trainer Constants.

/// An array of chessboard files as usize format 8 elements long.
pub const INT_FILES: [usize; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

/// An array of chessboard files as &str format 8 elements long, representing alpha.
pub const ALPHA_FILES: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

/// An array of chessboard files as &str format 8 elements long, representing numeric.
pub const ALPHA_RANKS: [&str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];

/// Compile in the version of this crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");