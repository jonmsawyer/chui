//! Chui: Board Error

use std::error::Error;
use std::fmt;

/// The main error that is returned for this application, rather than generic Err().
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum BoardError {
    InvalidFile(String),
    InvalidRank(String),
    InvalidTypeConversion(String),
}

/// Returns a string representing the particular [`BoardError`] variant.
impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoardError::InvalidFile(reason) => {
                write!(f, "Error (Invalid File): {}.", reason)
            }

            BoardError::InvalidRank(reason) => {
                write!(f, "Error (Invalid Rank): {}.", reason)
            }

            BoardError::InvalidTypeConversion(reason) => {
                write!(f, "Error (Invalid Type Conversion): {}.", reason)
            }
        }
    }
}

impl Error for BoardError {}

/// The main result type that is returned in this module, rather than the generic
/// `Result<T, E>` type.
pub type BoardResult<T> = std::result::Result<T, BoardError>;
