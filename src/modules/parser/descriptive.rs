#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use super::Parser;
use super::super::{Move, Piece};

/// A parser that will parse English descriptive chess notation.
/// Example moves: `P-K4`, `NxN`, `QxRch`, `Q-KR4 mate`, `O-O`, etc.
pub struct DescriptiveParser;

impl Parser for DescriptiveParser {
    fn parse(&self, the_move: &str, _board: &[[Option<Piece>; 8]; 8]) -> Move {
        Move::invalid(
            the_move,
            "Error: DescriptiveParser not implemented."
        )
    }
}

impl DescriptiveParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser> {
        Box::new(DescriptiveParser { })
    }
}
