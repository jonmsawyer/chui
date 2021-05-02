#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use super::Parser;
use super::super::{Move, Square};

/// A parser that will parse long algebraic chess notation.
/// Example moves: `e2e4`, `e7e5`, `d2d3`, `Bf8b4+`, `Bb5xc6`, etc.
pub struct LongAlgebraicParser;

impl Parser for LongAlgebraicParser {
    fn parse(&self, the_move: &str, _board: &[[Square; 8]; 8]) -> Move {
        Move::invalid(
            the_move,
            "Error: LongAlgebraicParser not implemented."
        )
    }
}

impl LongAlgebraicParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser> {
        Box::new(LongAlgebraicParser { })
    }
}
