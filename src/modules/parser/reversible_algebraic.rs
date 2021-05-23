#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use crate::{ChuiResult, ChuiError};
use super::Parser;
use super::super::Move;

/// A parser that will parse reversible algebraic chess notation.
/// Example moves: `e2-e4`, `e7-e5`, `Bb5xNc6`, `Bf8-b4#`, etc.
pub struct ReversibleAlgebraicParser;

impl Parser for ReversibleAlgebraicParser {
    /// Parse the chess move, return `Ok(Move)` on success,
    /// `ChuiError::InvalidMove(reason)` on failure.
    fn parse(&mut self, _the_move: &str)
    -> ChuiResult<Move>
    {
        Err(
            ChuiError::InvalidMove(
                "ReversibleAlgebraicParser not implemented".to_string()
            )
        )
    }

    fn name(&self) -> String {
        "Reversible Algebraic Parser".to_string()
    }

    fn eg(&self) -> String {
        format!("Examples for {}", self.name())
    }
}

impl ReversibleAlgebraicParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser> {
        Box::new(ReversibleAlgebraicParser { })
    }
}
