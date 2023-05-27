//! Chui Core's Coordinate base type.

pub use nonmax::NonMaxU8;

/// Main `Coord` struct used to represent chess piece and board position. We use non-max
/// u8 values because indicies are 0-indexed and values of 8 are invalid for an iterable
/// of size 7.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coord {
    rank: NonMaxU8,
    file: NonMaxU8,
}
