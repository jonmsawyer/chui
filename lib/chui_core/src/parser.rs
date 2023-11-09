//! `ParserEngine` struct.

#![allow(clippy::upper_case_acronyms)]

use std::fmt;

use crate::prelude::*;

// pub mod coordinate;
pub mod algebraic;
// pub mod concise_reversible;
// pub mod descriptive;
pub mod iccf;
// pub mod long_algebraic;
// pub mod reversible_algebraic;
// pub mod smith;

impl fmt::Debug for dyn Parser + Send + Sync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Parser").finish()
    }
}

/// Represents the different available supported parser engines for
/// chess moves.
#[derive(Debug, Default, Copy, Clone)]
pub enum ParserEngine {
    /// This engine variant helps to return an `AlgebraicParser`, which
    /// parses moves in algebraic notation.
    /// Example moves: `e4`, `Bxc6+`, `Kd6`, `e8Q#`, `a1=N`, etc.
    Algebraic,

    // /// This engine variant helps to return a `ConciseReversibleParser`,
    // /// which parses moves in concise reversible notation.
    // /// Example moves: `e24`, `e75`, `Ng1f3`, `Nb8c6`, `Bb5:Nc6`, etc.
    // ConciseReversible,

    // /// This engine variant helps to return a `CoordinateParser`,
    // /// which parses moves in Coordinate notation.
    // /// Example moves: `E2-E4`, `e7-e5`, `G1-F3`, `B8-c6`, `f1-b5`, etc.
    // Coordinate,

    // /// This engine variant helps to return a `DescriptiveParser`,
    // /// which parses moves in English descriptive notation.
    // /// Example moves: `P-K4`, `NxN`, `QxRch`, `Q-KR4 mate`, `O-O`, etc.
    // Descriptive,
    /// This engine variant helps to return a `ICCFParser`,
    /// which parses moves in ICCF notation.
    /// Example moves: `5254`, `5755`, `7163`, `2836`, `6125`, etc.
    #[default]
    ICCF,
    // /// This engine variant helps to return a `LongAlgebraicParser`,
    // /// which parses moves in long algebraic notation.
    // /// Example moves: `e2e4`, `e7e5`, `d2d3`, `Bf8b4+`, `Bb5xc6`, etc.
    // LongAlgebraic,

    // /// This engine variant helps to return a `ReversibleAlgebraicParser`,
    // /// which parses moves in reversible algebraic notation.
    // /// Example moves: `e2-e4`, `e7-e5`, `Bb5xNc6`, `Bf8-b4#`, etc.
    // ReversibleAlgebraic,

    // /// This engine variant helps to return a `SmithParser`,
    // /// which parses moves in Smith notation.
    // /// Example moves: `e1g1c`, `b4c3n`, `b5c6n`, `d7c6b`, `e2e4`, etc.
    // Smith,
}

impl ParserEngine {
    /// Given a variant of `ParserEngine`, this function returns a
    /// dynamic parser that implements the `Parse` trait.
    ///
    /// Example:
    ///
    /// ```
    /// use chui_core::{Game, Player, Color, parser::{self, ParserEngine}};
    ///
    /// let white = Player::new(
    ///     Color::White,
    ///     Some("Camina Drummer"),
    ///     Some(37),
    ///     None,
    /// );
    ///
    /// let black = Player::new(
    ///     Color::Black,
    ///     Some("Klaes Ashford"),
    ///     Some(72),
    ///     Some(1500),
    /// );
    ///
    /// let mut parser_engine = parser::new(ParserEngine::Algebraic);
    ///
    /// if let Ok(game) = Game::new(white, black, ParserEngine::Algebraic) {
    ///     println!("the move: {:?}", parser_engine.parse("P-K4".to_string(), Color::White));
    /// };
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new(parser: ParserEngine, to_move: Color) -> Box<dyn Parser + Send + Sync> {
        match parser {
            ParserEngine::Algebraic => algebraic::AlgebraicParser::new(to_move),
            // ParserEngine::ConciseReversible => concise_reversible::ConciseReversibleParser::new(),
            // ParserEngine::Coordinate => Coordinate::CoordinateParser::new(),
            // ParserEngine::Descriptive => descriptive::DescriptiveParser::new(),
            ParserEngine::ICCF => iccf::ICCFParser::new(to_move),
            // ParserEngine::LongAlgebraic => long_algebraic::LongAlgebraicParser::new(),
            // ParserEngine::ReversibleAlgebraic => reversible_algebraic::ReversibleAlgebraicParser::new(),
            // ParserEngine::Smith => smith::SmithParser::new(),
        }
    }
}
