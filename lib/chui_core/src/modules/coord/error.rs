//! Chui: Coord Error

use std::error::Error;
use std::fmt;

/// The main error that is returned for this application, rather than generic Err().
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum CoordError {
    InvalidFile(String),
    InvalidRank(String),
    InvalidTypeConversion(String),
}

/// Returns a string representing the particular `CoordError` variant.
impl fmt::Display for CoordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CoordError::InvalidFile(reason) => {
                write!(f, "Error (Invalid File): {}.", reason)
            }

            CoordError::InvalidRank(reason) => {
                write!(f, "Error (Invalid Rank): {}.", reason)
            }

            CoordError::InvalidTypeConversion(reason) => {
                write!(f, "Error (Invalid Type Conversion): {}.", reason)
            }
        }
    }
}

impl Error for CoordError {}
