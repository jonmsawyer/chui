#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use crate::{ChuiResult, ChuiError};
use super::Parser;
use super::super::{Move, Engine};

/// A parser that will parse English descriptive chess notation.
/// Example moves: `P-K4`, `NxN`, `QxRch`, `Q-KR4 mate`, `O-O`, etc.
pub struct DescriptiveParser;

impl Parser for DescriptiveParser {
    /// Parse the chess move, return `Ok(Move)` on success,
    /// `ChuiError::InvalidMove(reason)` on failure.
    fn parse(&mut self, _the_move: &str, _engine: &Engine)
    -> ChuiResult<Move>
    {
        Err(
            ChuiError::InvalidMove(
                "DescriptiveParser not implemented.".to_string()
            )
        )
    }
}

impl DescriptiveParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser> {
        Box::new(DescriptiveParser { })
    }
}
