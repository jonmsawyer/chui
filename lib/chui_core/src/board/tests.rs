#![cfg(test)]

mod test_standard_chess;
pub use test_standard_chess::standard_chess;

mod test_piece_coords;
pub use test_piece_coords::piece_coords;

mod test_bitmask_coords;
pub use test_bitmask_coords::bitmask_coords;
