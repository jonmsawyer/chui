#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use super::ParserEngineType;
use super::super::{Move, Square};

pub struct Parser {
    pub foo: String,
}

impl ParserEngineType for Parser {
    fn parse(&self, the_move: &str, _board: &[[Square; 8]; 8]) -> Move {
        Move::invalid(
            the_move,
            &format!("invalid at algebraic, foo is {}", self.foo)
        )
    }
}

impl Parser {
    pub fn new() -> Box<dyn ParserEngineType> {
        Box::new(Parser {
            foo: String::from("123"),
        })
    }
}
