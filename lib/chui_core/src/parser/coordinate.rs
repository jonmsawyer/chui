//! Coordinate notation module.

#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use crate::{ChuiError, ChuiResult, Color, Coord, Engine, Move};

use super::Parser;

/// A parser that will parse Coordinate chess notation.
/// Example moves: `E2-E4`, `e7-e5`, `G1-F3`, `B8-c6`, `f1-b5`, etc.
#[derive(Debug, Copy, Clone)]
pub struct CoordinateParser;

impl Parser for CoordinateParser {
    /// Parse the chess move, return `Ok(Move)` on success,
    /// `ChuiError::InvalidMove(reason)` on failure.
    fn parse(&mut self, _the_move: String, _to_move: Color) -> ChuiResult<Move> {
        Err(ChuiError::InvalidMove(
            "CoordinateParser not implemented".to_string(),
        ))
    }

    fn name(&self) -> String {
        "Coordinate Parser".to_string()
    }

    fn eg(&self) -> String {
        format!("Examples for {}", self.name())
    }

    /// Return a String representing the move from board Coordinates to this
    /// parser's notation.
    fn generate_move_from_board_Coordinates(
        &self,
        engine: &Engine,
        coord: Coord,
    ) -> ChuiResult<String> {
        let board = &(engine.board.get_board());

        let piece = match board[coord.get_rank()][coord.get_file()] {
            Some(piece) => piece,
            None => {
                return Err(ChuiError::InvalidMove(format!(
                    "Invalid move. No piece at ({}, {})",
                    coord.get_rank(),
                    coord.get_file()
                )))
            }
        };

        Ok(format!("Piece: {}", piece))
    }
}

impl CoordinateParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser + Send + Sync> {
        Box::new(CoordinateParser {})
    }
}
