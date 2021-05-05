//! Provides the enums for each chess `Piece` and `Color`.

use std::fmt;

/// Piece color. Either `White` or `Black` variants.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

/// Represents a piece on the chessboard. One of `King`, `Queen`,
/// `Rook`, `Knight`, `Bishop`, `Knight`, `Pawn`, or `None`.
///
/// Each chess piece has a `Color`.
///
/// Example:
///
/// ```
/// use chui::{Piece, Color};
/// 
/// let white_pawn = Piece::Pawn(Color::White);
/// let black_queen = Piece::Queen(Color::Black);
/// 
/// println!("White pawn: {:?}", white_pawn);
/// println!("Black queen: {:?}", black_queen);
/// ```
#[derive(Debug, Clone, Copy)]
pub enum Piece {
    /// A king with its `Color`.
    King(Color),

    /// A queen with its `Color`.
    Queen(Color),

    /// A rook with its `Color`.
    Rook(Color),

    /// A bishop with its `Color`.
    Bishop(Color),

    /// A knight with its `Color`.
    Knight(Color),

    /// A pawn with its `Color`.
    Pawn(Color),

    /// A None variant for `Piece::from(&str)`
    None,
}

/// Implements `Display` for `Piece`. Returns a string containing the string
/// representation of the chess piece. (e.g., "P" for a White Pawn.)
impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Piece::Pawn(Color::White) => write!(f, "P"),
            Piece::Rook(Color::White) => write!(f, "R"),
            Piece::Knight(Color::White) => write!(f, "N"),
            Piece::Bishop(Color::White) => write!(f, "B"),
            Piece::Queen(Color::White) => write!(f, "Q"),
            Piece::King(Color::White) => write!(f, "K"),
            Piece::Pawn(Color::Black) => write!(f, "p"),
            Piece::Rook(Color::Black) => write!(f, "r"),
            Piece::Knight(Color::Black) => write!(f, "n"),
            Piece::Bishop(Color::Black) => write!(f, "b"),
            Piece::Queen(Color::Black) => write!(f, "q"),
            Piece::King(Color::Black) => write!(f, "k"),
            Piece::None => write!(f, "·")
       }
   }
}

/// Cute little implementation of `object.from(&str)` of the `From` trait for
/// `Piece`. May never be needed within the crate, but here for convenience
/// if necessary.
impl From<&str> for Piece {
    fn from(piece: &str) -> Piece {
        match piece {
            "P" => Piece::Pawn(Color::White),
            "R" => Piece::Rook(Color::White),
            "N" => Piece::Knight(Color::White),
            "B" => Piece::Bishop(Color::White),
            "Q" => Piece::Queen(Color::White),
            "K" => Piece::King(Color::White),
            "p" => Piece::Pawn(Color::Black),
            "r" => Piece::Rook(Color::Black),
            "n" => Piece::Knight(Color::Black),
            "b" => Piece::Bishop(Color::Black),
            "q" => Piece::Queen(Color::Black),
            "k" => Piece::King(Color::Black),
            _ => Piece::None,
        }
    }
}
