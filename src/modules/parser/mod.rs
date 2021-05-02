use std::fmt;

use super::{Move, Square, Engine, Player, PieceColor};

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
    fn parse(&self, the_move: &str, board: &[[Square; 8]; 8]) -> Move;
}

impl fmt::Display for dyn ParserEngineType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let white = Player::new(
            PieceColor::White,
            "Drummer",
            Some("Camina"),
            None,
            None,
            Some(37),
            None,
        );
    
        let black = Player::new(
            PieceColor::Black,
            "Ashford",
            Some("Klaes"),
            None,
            None,
            Some(72),
            Some(1500),
        );
        
        let engine = Engine::new(white, black);

        write!(
            f,
            "(ParserEngineType) parse is {}",
            self.parse("ParseEngineType move", &engine.board)
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
