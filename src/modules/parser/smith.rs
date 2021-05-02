#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use super::Parser;
use super::super::{Move, Square};

/// A parser that will parse Smith chess notation.
/// Example moves: `e1g1c`, `b4c3n`, `b5c6n`, `d7c6b`, `e2e4`, etc.
pub struct SmithParser;

impl Parser for SmithParser {
    fn parse(&self, the_move: &str, _board: &[[Square; 8]; 8]) -> Move {
        Move::invalid(
            the_move,
            "Error: SmithParser not implemented."
        )
    }
}

impl SmithParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser> {
        Box::new(SmithParser { })
    }
}
