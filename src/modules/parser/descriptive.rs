#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use super::super::{Color, Move};
use super::Parser;
use crate::{ChuiError, ChuiResult};

/// A parser that will parse English descriptive chess notation.
/// Example moves: `P-K4`, `NxN`, `QxRch`, `Q-KR4 mate`, `O-O`, etc.
pub struct DescriptiveParser;

impl Parser for DescriptiveParser {
    /// Parse the chess move, return `Ok(Move)` on success,
    /// `ChuiError::InvalidMove(reason)` on failure.
    fn parse(&mut self, _the_move: String, _to_move: Color) -> ChuiResult<Move> {
        Err(ChuiError::InvalidMove(
            "DescriptiveParser not implemented".to_string(),
        ))
    }

    fn name(&self) -> String {
        "Descriptive Parser".to_string()
    }

    fn eg(&self) -> String {
        format!("Examples for {}", self.name())
    }
}

impl DescriptiveParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser + Send + Sync> {
        Box::new(DescriptiveParser {})
    }
}
