//! The main library for Chui.
//!
//! The application is defined in a library, rather than directly in the binary source,
//! in part so that tests can be more easily run against it. Writing it as a library
//! also gives us the ability to easily create other binaries that run parts of it
//! here.

#![allow(clippy::nonminimal_bool, clippy::print_stdout, clippy::use_debug)]
#![warn(
    clippy::cognitive_complexity,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::float_equality_without_abs,
    keyword_idents,
    clippy::missing_const_for_fn,
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::mod_module_files,
    non_ascii_idents,
    noop_method_call,
    clippy::option_if_let_else,
    clippy::print_stderr,
    clippy::semicolon_if_nothing_returned,
    clippy::unseparated_literal_suffix,
    clippy::shadow_unrelated,
    clippy::suspicious_operation_groupings,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    clippy::unused_self,
    clippy::used_underscore_binding,
    clippy::useless_let_if_seq,
    clippy::wildcard_dependencies,
    clippy::wildcard_imports
)]

use std::fmt;

#[macro_use]
mod macros;

mod modules;
pub use modules::{
    parser::{self, ParserEngine},
    piece::{Color, Piece, PieceKind},
    Board, ChessVariant, Command, CommandContext, CommandKind, Engine, Fen, Move, MoveGenerator,
    MoveType, Player, Ui,
};

/// The main error that is returned for this application, rather than generic Err().
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
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

    /// When generating a move string from board coordinates, the coordinates
    /// must be within a valid range (0-7).
    IndexOutOfRange(String),

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
            } // ChuiError::Unknown(reason) => {
              //     write!(f, "Error (Unknown): {}", reason)
              // }
        }
    }
}

/// The main result type that is returned in this application, rather than the
/// generic Ok().
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
