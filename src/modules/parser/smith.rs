#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use crate::{ChuiResult, ChuiError};
use super::Parser;
use super::super::Move;

/// A parser that will parse Smith chess notation.
/// Example moves: `e1g1c`, `b4c3n`, `b5c6n`, `d7c6b`, `e2e4`, etc.
pub struct SmithParser;

impl Parser for SmithParser {
    /// Parse the chess move, return `Ok(Move)` on success,
    /// `ChuiError::InvalidMove(reason)` on failure.
    fn parse(&mut self, _the_move: &str)
    -> ChuiResult<Move>
    {
        Err(
            ChuiError::InvalidMove(
                "SmithParser not implemented".to_string()
            )
        )
    }

    fn name(&self) -> String {
        "Smith Parser".to_string()
    }

    fn eg(&self) -> String {
        format!("Examples for {}", self.name())
    }
}

impl SmithParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser> {
        Box::new(SmithParser { })
    }
}
