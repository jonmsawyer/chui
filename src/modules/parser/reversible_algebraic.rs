#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use super::Parser;
use super::super::{Move, Piece};

/// A parser that will parse reversible algebraic chess notation.
/// Example moves: `e2-e4`, `e7-e5`, `Bb5xNc6`, `Bf8-b4#`, etc.
pub struct ReversibleAlgebraicParser;

impl Parser for ReversibleAlgebraicParser {
    fn parse(&self, the_move: &str, _board: &[[Option<Piece>; 8]; 8]) -> Move {
        Move::invalid(
            the_move,
            "Error: ReversibleAlgebraicParser not implemented."
        )
    }
}

impl ReversibleAlgebraicParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser> {
        Box::new(ReversibleAlgebraicParser { })
    }
}
