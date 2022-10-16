#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use crate::{ChuiResult, ChuiError};
use super::Parser;
use super::super::{Move, Color};

/// A parser that will parse ICCF chess notation.
/// Example moves: `5254`, `5755`, `7163`, `2836`, `6125`, etc.
pub struct ICCFParser;

impl Parser for ICCFParser {
    /// Parse the chess move, return `Ok(Move)` on success,
    /// `ChuiError::InvalidMove(reason)` on failure.
    fn parse(&mut self, _the_move: String, _to_move: Color)
    -> ChuiResult<Move>
    {
        Err(
            ChuiError::InvalidMove(
                "ICCFParser not implemented".to_string()
            )
        )
    }

    fn name(&self) -> String {
        "ICCF Parser".to_string()
    }

    fn eg(&self) -> String {
        format!("Examples for {}", self.name())
    }
}

impl ICCFParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser + Send + Sync> {
        Box::new(ICCFParser { })
    }
}
