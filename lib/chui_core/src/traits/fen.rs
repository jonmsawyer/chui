//! FEN notation module.
//!
//! FEN stands for Forsyth-Edwards Notation.

use crate::prelude::*;

/// Represents the FEN notation of a chess position.
pub trait Fen {
    /// Get the FEN layout of the board.
    fn get_fen(board: &Board) -> String;

    /// Get the Shredder-FEN layout of the board.
    fn get_shredder_fen(board: &Board) -> String;

    /// Get the X-FEN layout of the board.
    fn get_x_fen(board: &Board) -> String;

    /// Get FEN layout of the board only without the other
    /// attributes.
    fn get_board_fen(board: &Board) -> String;
}
