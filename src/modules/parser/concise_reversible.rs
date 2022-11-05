#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use super::super::{Color, Move};
use super::Parser;
use crate::{ChuiError, ChuiResult};

/// A parser that will parse concise reversible chess notation.
/// Example moves: `e24`, `e75`, `Ng1f3`, `Nb8c6`, `Bb5:Nc6`, etc.
pub struct ConciseReversibleParser;

impl Parser for ConciseReversibleParser {
    /// Parse the chess move, return `Ok(Move)` on success,
    /// `ChuiError::InvalidMove(reason)` on failure.
    fn parse(&mut self, _the_move: String, _to_move: Color) -> ChuiResult<Move> {
        Err(ChuiError::InvalidMove(
            "ConciseReversibleParser not implemented".to_string(),
        ))
    }

    fn name(&self) -> String {
        "Concise Reversible Parser".to_string()
    }

    fn eg(&self) -> String {
        format!("Examples for {}", self.name())
    }
}

impl ConciseReversibleParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser + Send + Sync> {
        Box::new(ConciseReversibleParser {})
    }
}
