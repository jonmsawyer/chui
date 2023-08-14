//! Chui: Result and Error

#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]

use std::convert::Infallible;
use std::fmt;
use std::num::{ParseIntError, TryFromIntError};

// use nonmax;
use thiserror::Error;

/// The main error that is returned for this application, rather than generic Err().
#[non_exhaustive]
#[derive(Error, Debug, PartialEq, Eq, Clone, Hash)]
pub enum ChuiError {
    /// Invalid input if the input string is too small or too large, or
    /// if the input move has any interim whitespace.
    InvalidInput(String),

    /// An invalid move. This variant shows up when the user tries to
    /// make an invalid move on the chess board, usually in these ways:
    ///
    /// 1. There's no piece in the "from" square
    /// 2. There's a friendly piece blocking the move
    /// 3. Player's king is in check
    /// 4. Player's king would get into check
    /// 5. The move is simply invalid according to the rules
    /// 6. etc.
    InvalidMove(String),

    /// An invalid piece. This variant shows up when the consumer of this
    /// crate tries to intialize a `Piece` using the `try_from(&str)`
    /// method using an invalid `&str`. `&str` must be one of
    /// \[PKQRBNpkqrbn\].
    InvalidPiece(String),

    /// Incompatible sides. This variant shows up when an `Engine` is
    /// initialized with `player_1` and `player_2` being the same `Color`.
    IncompatibleSides(String),

    /// When parsing a move, this variant shows up when a token's processing
    /// logic has not been satisfied. When writing a parser, the goal is to
    /// never see this error.
    TokenNotSatisfied(String),

    /// When generating a move string from board Coordinates, the Coordinates
    /// must be within a valid range (0-7).
    IndexOutOfRange(String),

    /// Invalid rank.
    InvalidRank(String),

    /// Invalid file.
    InvalidFile(String),

    /// Invalid coordinates.
    InvalidCoords(String),

    /// The square at a coordinate is occupied.
    SquareOccupied(String),

    /// Invalid type conversion.
    InvalidTypeConversion(String),

    /// The [`TryFromIntError`] type.
    TryFromIntError(String),

    /// The [`ParseIntError`] type.
    ParseIntError(String),

    /// Something is not implemented completely. Raise this error when in
    /// development/testing.
    NotImplemented(String),

    /// Unknown error. Used for testing.
    Unknown(String),
}

/// Returns a string representing the particular `ChuiError` variant.
impl fmt::Display for ChuiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChuiError::InvalidInput(reason) => {
                write!(f, "Error (Invalid Input): {}.", reason)
            }

            ChuiError::InvalidMove(reason) => {
                write!(f, "Error (Invalid Move): {}.", reason)
            }

            ChuiError::InvalidPiece(reason) => {
                write!(f, "Error (Invalid Piece): {}.", reason)
            }

            ChuiError::IncompatibleSides(reason) => {
                write!(f, "Error (Incompatible Sides): {}.", reason)
            }

            ChuiError::TokenNotSatisfied(reason) => {
                write!(f, "Error (Token Not Satisfied): {}.", reason)
            }

            ChuiError::IndexOutOfRange(reason) => {
                write!(f, "Error (Index Out Of Range): {}.", reason)
            }

            ChuiError::NotImplemented(reason) => {
                write!(f, "Error (Not Implemented): {}.", reason)
            }

            ChuiError::InvalidRank(reason) => {
                write!(f, "Error (Invalid Rank): {}.", reason)
            }

            ChuiError::InvalidFile(reason) => {
                write!(f, "Error (Invalid File): {}.", reason)
            }

            ChuiError::InvalidCoords(reason) => {
                write!(f, "Error (Invalid Coordinates): {}.", reason)
            }

            ChuiError::SquareOccupied(reason) => {
                write!(f, "Error (Square Occupied): {}.", reason)
            }

            ChuiError::InvalidTypeConversion(reason) => {
                write!(f, "Error (Invalid File): {}.", reason)
            }

            ChuiError::TryFromIntError(reason) => {
                write!(f, "Error (TryFromIntError): {}.", reason)
            }

            ChuiError::ParseIntError(reason) => {
                write!(f, "Error (ParseIntError): {}.", reason)
            }

            ChuiError::Unknown(reason) => {
                write!(f, "Error (Unknown): {}", reason)
            }
        }
    }
}

impl From<TryFromIntError> for ChuiError {
    fn from(error: TryFromIntError) -> ChuiError {
        ChuiError::TryFromIntError(format!("Could not parse integer from input: {}", error))
    }
}

impl From<ParseIntError> for ChuiError {
    fn from(error: ParseIntError) -> ChuiError {
        ChuiError::TryFromIntError(format!("Could not parse integer from input: {}", error))
    }
}

impl From<nonmax::TryFromIntError> for ChuiError {
    fn from(error: nonmax::TryFromIntError) -> ChuiError {
        ChuiError::TryFromIntError(format!(
            "Could not parse non-max integer from input: {}",
            error
        ))
    }
}

impl From<nonmax::ParseIntError> for ChuiError {
    fn from(error: nonmax::ParseIntError) -> ChuiError {
        ChuiError::TryFromIntError(format!(
            "Could not parse non-max integer from input: {}",
            error
        ))
    }
}

impl From<Infallible> for ChuiError {
    fn from(inf: Infallible) -> ChuiError {
        ChuiError::Unknown(format!("Unknown error: {:?}", inf))
    }
}

impl<T: fmt::Debug> From<Result<T, ChuiError>> for ChuiError {
    fn from(result: Result<T, ChuiError>) -> ChuiError {
        result.unwrap_err()
    }
}

/// The main result type that is returned in this application, rather than the
/// generic Ok().
pub type ChuiResult<T> = std::result::Result<T, ChuiError>;