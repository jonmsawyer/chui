#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use super::ParserEngineType;
use super::super::{Move, Square};

pub struct Parser {
    pub bar: String,
}

impl ParserEngineType for Parser {
    fn parse(&self, the_move: &str, _board: &[[Square; 8]; 8]) -> Move {
        Move::invalid(
            the_move,
            &format!("invalid at coordinate, bar is {}", self.bar)
        )
    }
}

impl Parser {
    pub fn new() -> Box<dyn ParserEngineType> {
        Box::new(Parser {
            bar: String::from("456"),
        })
    }
}
