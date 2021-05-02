use std::fmt;

use super::Move;

pub mod algebraic;
// mod long_algebraic;
// mod reversible_algebraic;
// mod concise_reversible;
// mod smith;
// mod descriptive;
pub mod coordinate;
// mod iccf;
// mod invalid;

pub trait ParserEngineType {
    fn parse(&self, the_move: &str) -> Move;
}

impl fmt::Display for dyn ParserEngineType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(ParserEngineType) parse is {}",
            self.parse("ParseEngineType move")
        )
    }
}

#[derive(Debug)]
pub enum ParserEngine {
    Algebraic,
    // LongAlgebraic,
    // ReverisbleAlgebraic,
    // ConciseReversible,
    // Smith,
    // Descriptive,
    Coordinate,
    // Iccf,
    // Invalid,
}

pub fn new(parser: ParserEngine) -> Box<dyn ParserEngineType> {
    match parser {
        ParserEngine::Algebraic => algebraic::Parser::new(),
        ParserEngine::Coordinate => coordinate::Parser::new(),
    }
}
