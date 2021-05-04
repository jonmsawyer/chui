//! Provides the enum for each chess piece. `Pawn`, `Rook`, `Knight`,
//! `Bishop`, `Queen`, `King`, and `None`.

/// Piece color. Either `White` or `Black` variants.
#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

/// Represents a piece on the chessboard. One of `King`, `Queen`,
/// `Rook`, `Knight`, `Bishop`, `Knight`, or `Pawn`.
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
#[derive(Debug)]
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

    /// No piece at all.
    None,
}
