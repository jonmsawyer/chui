#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use crate::{ChuiResult, ChuiError};
use super::Parser;
use super::super::{Move, Engine};

/// A parser that will parse long algebraic chess notation.
/// Example moves: `e2e4`, `e7e5`, `d2d3`, `Bf8b4+`, `Bb5xc6`, etc.
pub struct LongAlgebraicParser;

impl Parser for LongAlgebraicParser {
    /// Parse the chess move, return `Ok(Move)` on success,
    /// `ChuiError::InvalidMove(reason)` on failure.
    fn parse(&mut self, _the_move: &str, _engine: &Engine)
    -> ChuiResult<Move>
    {
        Err(
            ChuiError::InvalidMove(
                "LongAlgebraicParser not implemented.".to_string()
            )
        )
    }
}

impl LongAlgebraicParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser> {
        Box::new(LongAlgebraicParser { })
    }
}
