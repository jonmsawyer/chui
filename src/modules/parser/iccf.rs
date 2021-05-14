#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use crate::{ChuiResult, ChuiError};
use super::Parser;
use super::super::{Move, Engine};

/// A parser that will parse ICCF chess notation.
/// Example moves: `5254`, `5755`, `7163`, `2836`, `6125`, etc.
pub struct ICCFParser;

impl Parser for ICCFParser {
    /// Parse the chess move, return `Ok(Move)` on success,
    /// `ChuiError::InvalidMove(reason)` on failure.
    fn parse(&self, _the_move: &str, _engine: &Engine)
    -> ChuiResult<Move>
    {
        Err(
            ChuiError::InvalidMove(
                "ICCFParser not implemented.".to_string()
            )
        )
    }
}

impl ICCFParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser> {
        Box::new(ICCFParser { })
    }
}
