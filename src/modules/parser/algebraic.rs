#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use super::Parser;
use super::super::{Move, Square};

/// A parser that will parse algebraic chess notation.
/// Example moves: `e4`, `Bxc6+`, `Kd6`, `e8Q#`, `a1=N`, etc.
pub struct AlgebraicParser;

impl Parser for AlgebraicParser {
    fn parse(&self, the_move: &str, _board: &[[Square; 8]; 8]) -> Move {
        Move::invalid(
            the_move,
            "Error: AlgebraicParser not implemented."
        )
    }
}

impl AlgebraicParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser> {
        Box::new(AlgebraicParser { })
    }
}
