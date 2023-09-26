#![cfg(test)]

pub mod test_standard_chess;
pub use test_standard_chess::standard_chess;

pub mod test_piece_coords;
pub use test_piece_coords::piece_coords;

pub mod test_bitmask_coords;
pub use test_bitmask_coords::bitmask_coords;
