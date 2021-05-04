#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use super::Parser;
use super::super::{Move, Piece};

/// A parser that will parse ICCF chess notation.
/// Example moves: `5254`, `5755`, `7163`, `2836`, `6125`, etc.
pub struct ICCFParser;

impl Parser for ICCFParser {
    fn parse(&self, the_move: &str, _board: &[[Option<Piece>; 8]; 8]) -> Move {
        Move::invalid(
            the_move,
            "Error: ICCFParser not implemented."
        )
    }
}

impl ICCFParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new() -> Box<dyn Parser> {
        Box::new(ICCFParser { })
    }
}
