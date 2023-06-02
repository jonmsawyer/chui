//! Concise Reversible notation module.

#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use super::super::{Color, Engine, Move};
use super::Parser;
use crate::{ChuiError, ChuiResult};

/// A parser that will parse concise reversible chess notation.
/// Example moves: `e24`, `e75`, `Ng1f3`, `Nb8c6`, `Bb5:Nc6`, etc.
#[derive(Debug, Copy, Clone)]
pub struct ConciseReversibleParser;

impl Parser for ConciseReversibleParser {
    /// Parse the chess move, return `Ok(Move)` on success,
    /// `ChuiError::InvalidMove(reason)` on failure.
    fn parse(&mut self, _the_move: String, _to_move: Color) -> ChuiResult<Move> {
        Err(ChuiError::InvalidMove(
            "ConciseReversibleParser not implemented".to_string(),
        ))
    }

    fn name(&self) -> String {
        "Concise Reversible Parser".to_string()
    }

    fn eg(&self) -> String {
        format!("Examples for {}", self.name())
    }

    /// Return a String representing the move from board Coordinates to this
    /// parser's notation.
    fn generate_move_from_board_Coordinates(
        &self,
        engine: &Engine,
        from_index: (Coord),
        _to_index: (Coord),
    ) -> ChuiResult<String> {
        let board = &(engine.board.get_board());

        let piece = match board[from_index.0][from_index.1] {
            Some(piece) => piece,
            None => {
                return Err(ChuiError::InvalidMove(format!(
                    "Invalid move. No piece at ({}, {})",
                    from_index.0, from_index.1
                )))
            }
        };

        Ok(format!("Piece: {}", piece))
    }
}

impl ConciseReversibleParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser + Send + Sync> {
        Box::new(ConciseReversibleParser {})
    }
}
