//! Reverse Algebraic notation module.

#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use super::super::{Color, Engine, Move};
use super::Parser;
use crate::{ChuiError, ChuiResult};

/// A parser that will parse reversible algebraic chess notation.
/// Example moves: `e2-e4`, `e7-e5`, `Bb5xNc6`, `Bf8-b4#`, etc.
#[derive(Debug, Copy, Clone)]
pub struct ReversibleAlgebraicParser;

impl Parser for ReversibleAlgebraicParser {
    /// Parse the chess move, return `Ok(Move)` on success,
    /// `ChuiError::InvalidMove(reason)` on failure.
    fn parse(&mut self, _the_move: String, _to_move: Color) -> ChuiResult<Move> {
        Err(ChuiError::InvalidMove(
            "ReversibleAlgebraicParser not implemented".to_string(),
        ))
    }

    fn name(&self) -> String {
        "Reversible Algebraic Parser".to_string()
    }

    fn eg(&self) -> String {
        format!("Examples for {}", self.name())
    }

    /// Return a String representing the move from board Coordinates to this
    /// parser's notation.
    fn generate_move_from_board_Coordinates(
        &self,
        engine: &Engine,
        from_index: Coord,
        _to_index: Coord,
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

impl ReversibleAlgebraicParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser + Send + Sync> {
        Box::new(ReversibleAlgebraicParser {})
    }
}
