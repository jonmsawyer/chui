//! Chui Core's Coordinate base type.

use std::str::FromStr;

pub use nonmax::{NonMaxU8, ParseIntError, TryFromIntError};

/// Main `Coord` struct used to represent chess piece and board position. We use non-max
/// u8 values because indicies are 0-indexed and values of 8 are invalid for an iterable
/// of size 7.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coord {
    rank: NonMaxU8,
    file: NonMaxU8,
}

impl TryFrom<(u8, u8)> for Coord {
    type Error = TryFromIntError;

    fn try_from(coord: (u8, u8)) -> Result<Coord, TryFromIntError> {
        Ok(Coord {
            rank: NonMaxU8::try_from(coord.0)?,
            file: NonMaxU8::try_from(coord.1)?,
        })
    }
}

impl TryFrom<(&str, &str)> for Coord {
    type Error = ParseIntError;

    fn try_from(coord: (&str, &str)) -> Result<Coord, ParseIntError> {
        Ok(Coord {
            rank: NonMaxU8::from_str(coord.0)?,
            file: NonMaxU8::from_str(coord.1)?,
        })
    }
}

impl TryFrom<(char, char)> for Coord {
    type Error = nonmax::ParseIntError;

    fn try_from(coord: (char, char)) -> Result<Coord, nonmax::ParseIntError> {
        Ok(Coord {
            rank: NonMaxU8::from_str(&coord.0.to_string())?,
            file: NonMaxU8::from_str(&coord.1.to_string())?,
        })
    }
}
