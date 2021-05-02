#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use super::Parser;
use super::super::{Move, Square};

/// A parser that will parse concise reversible chess notation.
/// Example moves: `e24`, `e75`, `Ng1f3`, `Nb8c6`, `Bb5:Nc6`, etc.
pub struct ConciseReversibleParser;

impl Parser for ConciseReversibleParser {
    fn parse(&self, the_move: &str, _board: &[[Square; 8]; 8]) -> Move {
        Move::invalid(
            the_move,
            "Error: ConciseReversibleParser not implemented."
        )
    }
}

impl ConciseReversibleParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser> {
        Box::new(ConciseReversibleParser {})
    }
}
