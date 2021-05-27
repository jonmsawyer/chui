use std::fmt;

mod modules;
pub use modules::{
    Engine, Player, Board, ChessVariant,
    Move, MoveType, MoveGenerator,
    piece::{Piece, PieceKind, Color},
    parser::{self, ParserEngine},
    Command, CommandContext, CommandKind,
    Fen,
};


#[non_exhaustive]
#[derive(Debug, PartialEq)]
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

    /// Something is not implemented completely. Raise this error when in
    /// development/testing.
    NotImplemented(String),

    // /// Unknown error. Used for testing.
    // Unknown(String),
}

/// Returns a string representing the particular `ChuiError` variant.
impl fmt::Display for ChuiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChuiError::InvalidInput(reason) => {
                write!(f, "Error (Invalid Input): {}.", reason)
            },

            ChuiError::InvalidMove(reason) => {
                write!(f, "Error (Invalid Move): {}.", reason)
            },

            ChuiError::InvalidPiece(reason) => {
                write!(f, "Error (Invalid Piece): {}.", reason)
            },

            ChuiError::IncompatibleSides(reason) => {
                write!(f, "Error (Incompatible Sides): {}.", reason)
            },

            ChuiError::TokenNotSatisfied(reason) => {
                write!(f, "Error (Token Not Satisfied): {}.", reason)
            },

            ChuiError::NotImplemented(reason) => {
                write!(f, "Error (Not Implemented): {}.", reason)
            },

            // ChuiError::Unknown(reason) => {
            //     write!(f, "Error (Unknown): {}", reason)
            // }
        }
    }
}

pub type ChuiResult<T> = std::result::Result<T, ChuiError>;

// Keep for now. May never need it anymore.
// #[macro_export]
// macro_rules! hashmap {
//     ($( $key: expr => $val: expr ),*) => {{
//          let mut map = ::std::collections::HashMap::new();
//          $( map.insert($key, $val); )*
//          map
//     }}
// }
