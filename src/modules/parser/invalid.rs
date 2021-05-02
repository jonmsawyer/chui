#![allow(clippy::new_ret_no_self)]

//use std::fmt;

use super::ParserEngineType;

pub struct Parser {
    pub bar: String,
}

impl ParserEngineType for Parser {
    fn parse(&self) -> String {
        String::from("Invalid parser.")
    }
}

impl Parser {
    pub fn new() -> Box<dyn ParserEngineType> {
        Box::new(Parser {
            bar: String::from("bar"),
        })
    }
}
