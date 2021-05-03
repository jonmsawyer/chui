//! File: `square.rs`
//!
//! Module: `square`
//!
//! Provides the struct for `Square`. Each `Square` has a `coord`,
//! `piece`, and `color`.

use super::color::SquareColor;
use super::piece::Piece;

/// Contains the information related to a `Square` on the
/// chessboard, such as `coordinates`, `piece`, and square
/// `color`.
///
/// Example:
///
/// ```
/// use chui::{
///     Square, Piece,
///     SquareColor, PieceColor,
/// };
/// 
/// let square = Square {
///     coord: ('a', 1),
///     piece: Piece::Rook(PieceColor::White),
///     color: SquareColor::Dark,
/// };
/// 
/// println!("{:?}", square);
/// ```
#[derive(Debug)]
pub struct Square {
    /// The coordinate of the square (e.g., ('a', 1)).
    pub coord: (char, u8),

    /// The `Piece` that occupies the square. If none, then
    /// `Piece::None`.
    pub piece: Piece,

    /// The square color of the square. One of `Light` and
    /// `Dark`.
    pub color: SquareColor,
}
