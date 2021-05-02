//! File: `piece.rs`
//!
//! Module: `piece`
//!
//! Provides the enum for each chess piece. `Pawn`, `Rook`, `Knight`,
//! `Bishop`, `Queen`, `King`, and `None`.

use super::color::PieceColor;

/// Represents a piece on the chessboard. One of `King`, `Queen`,
/// `Rook`, `Knight`, `Bishop`, `Knight`, or `Pawn`.
///
/// Each chess piece has a `PieceColor`.
///
/// Example:
///
/// ```
/// use chui::{Piece, PieceColor};
/// 
/// let white_pawn = Piece::Pawn(PieceColor::White);
/// let black_queen = Piece::Queen(PieceColor::Black);
/// 
/// println!("White pawn: {:?}", white_pawn);
/// println!("Black queen: {:?}", black_queen);
/// ```
#[derive(Debug)]
pub enum Piece {
    /// A king with its `PieceColor`.
    King(PieceColor),
    /// A queen with its `PieceColor`.
    Queen(PieceColor),
    /// A rook with its `PieceColor`.
    Rook(PieceColor),
    /// A bishop with its `PieceColor`.
    Bishop(PieceColor),
    /// A knight with its `PieceColor`.
    Knight(PieceColor),
    /// A pawn with its `PieceColor`.
    Pawn(PieceColor),
    /// No piece at all.
    None,
}
