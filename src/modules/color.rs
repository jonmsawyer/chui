//! File: `color.rs`
//!
//! Module: `color`
//!
//! Provides enums for the colors of pieces and squares.

/// Piece color. Either `White` or `Black` variants.
#[derive(Debug, Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

/// Square color. Either `Light` or `Dark` variants.
#[derive(Debug)]
pub enum SquareColor {
    Light,
    Dark,
}
