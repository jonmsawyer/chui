#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use crate::{ChuiResult, ChuiError};
use super::Parser;
use super::super::{Move, Engine};

/// A parser that will parse coordinate chess notation.
/// Example moves: `E2-E4`, `e7-e5`, `G1-F3`, `B8-c6`, `f1-b5`, etc.
pub struct CoordinateParser;

impl Parser for CoordinateParser {
    /// Parse the chess move, return `Ok(Move)` on success,
    /// `ChuiError::InvalidMove(reason)` on failure.
    fn parse(&mut self, _the_move: &str, _engine: &Engine)
    -> ChuiResult<Move>
    {
        Err(
            ChuiError::InvalidMove(
                "CoordinateParser not implemented.".to_string()
            )
        )
    }
}

impl CoordinateParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser> {
        Box::new(CoordinateParser { })
    }
}
