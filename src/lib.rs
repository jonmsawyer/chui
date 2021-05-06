use std::fmt;

mod modules;
pub use modules::{
    Engine, Move, MoveGenerator, Player,
    piece::{Piece, Color},
    parser::{self, ParserEngine},
};

#[non_exhaustive]
#[derive(Debug)]
pub enum ChuiError {
    /// An invalid move. This variant shows up when the user tries to
    /// make an invalid move on the chess board, usually in these ways:
    /// 
    /// 1. There's no piece in the "from" square
    /// 1. There's a friendly piece blocking the move
    /// 1. Player's king is in check
    /// 1. Player's king would get into check
    /// 1. The move is simply invalid according to the rules
    /// 1. etc.
    InvalidMove(String),

    /// An invalid piece. This variant shows up when the consumer of this
    /// crate tries to intialize a `Piece` using the `try_from(&str)`
    /// method using an invalid `&str`. `&str` must be one of
    /// \[PKQRBNpkqrbn\].
    InvalidPiece(String),

    /// Incompatible sides. This variant shows up when an `Engine` is
    /// initialized with `player_1` and `player_2` being the same `Color`.
    IncompatibleSides(String),
}

/// Returns a string representing the particular `ChuiError` variant.
impl fmt::Display for ChuiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChuiError::InvalidMove(reason) => {
                write!(f, "Error (Invalid Move): {}.", reason)
            },
            ChuiError::InvalidPiece(reason) => {
                write!(f, "Error (Invalid Piece): {}.", reason)
            },
            ChuiError::IncompatibleSides(reason) => {
                write!(f, "Error (Incompatible Sides): {}.", reason)
            },
        }
    }
}

type Result<T> = std::result::Result<T, ChuiError>;
