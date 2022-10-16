//! Provides a trait for `Parser` instances. Each parser implements
//! this `Parser`. Must implement the `parse`, `name`, and `eg`
//! methods.

#![allow(clippy::upper_case_acronyms)]

use std::fmt;

use crate::{ChuiResult, ChuiError};
use super::{Move, Color};

pub mod algebraic;
pub mod long_algebraic;
pub mod reversible_algebraic;
pub mod concise_reversible;
pub mod smith;
pub mod descriptive;
pub mod coordinate;
pub mod iccf;

/// Implement this trait to define the `parse()` method on a parser.
/// Any struct implementing this trait should parse a chess move
/// in an expected notation and return a `Move` object, representing
/// the validty or invalidity of the requested move for the given
/// chessboard.
///
/// Example:
///
/// ```
/// use chui::{
///     Move, Color, Piece, parser::Parser,
///     ChuiResult, ChuiError, Engine
/// };
///
/// pub struct MyParser;
///
/// impl Parser for MyParser {
///     fn parse(&mut self, _the_move: &str, _color: Color)
///     -> ChuiResult<Move>
///     {
///         Err(
///             ChuiError::InvalidMove(
///                 "MyParser not implemented.".to_string()
///             )
///         )
///     }
///
///     fn name(&self) -> String {
///         "My Parser".to_string()
///     }
///
///     fn eg(&self) -> String {
///         "My Parser example moves".to_string()
///     }
/// }
/// ```
pub trait Parser: Send + Sync {
    /// Parse the chess move, return `Ok(Move)` on success,
    /// `ChuiError::InvalidMove(reason)` on failure.
    fn parse(&mut self, the_move: String, to_move: Color)
    -> ChuiResult<Move>;

    /// The name of the parser. Used in help messages and debug.
    fn name(&self) -> String;

    /// Example inputs.
    fn eg(&self) -> String;

    /// Trim the whitespace from `the_move` and check to see that
    /// the move doesn't contain any whitespace after the trim.
    fn trim_and_check_whitespace(&self, the_move: String)
    -> ChuiResult<String>
    {
        let the_move = the_move.trim().to_string();

        if the_move.eq("") {
            self.invalid_input("Input move cannot be empty")?;
        }

        if the_move.contains(char::is_whitespace) {
            self.invalid_input("Input move contains whitespace")?
        }

        Ok(the_move)
    }

    /// Match the given file (`char`) to its index (`u8`).
    fn match_file_to_index(&self, file: char) -> Option<u8> {
        match file {
            'a' => Some(0),
            'b' => Some(1),
            'c' => Some(2),
            'd' => Some(3),
            'e' => Some(4),
            'f' => Some(5),
            'g' => Some(6),
            'h' => Some(7),
            _ => None,
        }
    }

    /// Match the given rank (`char`) to its index (`u8`).
    fn match_rank_to_index(&self, rank: char) -> Option<u8> {
        match rank {
            '1' => Some(0),
            '2' => Some(1),
            '3' => Some(2),
            '4' => Some(3),
            '5' => Some(4),
            '6' => Some(5),
            '7' => Some(6),
            '8' => Some(7),
            _ => None,
        }
    }

    fn invalid_input(&self, reason: &str) -> ChuiResult<()> {
        Err(ChuiError::InvalidInput(reason.to_string()))
    }

}

impl fmt::Debug for dyn Parser + Send + Sync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Parser")
            .finish()
    }
}

/// Represents the different available supported parser engines for
/// chess moves.
#[derive(Debug)]
pub enum ParserEngine {
    /// This engine variant helps to return an `AlgebraicParser`, which
    /// parses moves in algebraic notation.
    /// Example moves: `e4`, `Bxc6+`, `Kd6`, `e8Q#`, `a1=N`, etc.
    Algebraic,

    /// This engine variant helps to return a `ConciseReversibleParser`,
    /// which parses moves in concise reversible notation.
    /// Example moves: `e24`, `e75`, `Ng1f3`, `Nb8c6`, `Bb5:Nc6`, etc.
    ConciseReversible,

    /// This engine variant helps to return a `CoordinateParser`,
    /// which parses moves in coordinate notation.
    /// Example moves: `E2-E4`, `e7-e5`, `G1-F3`, `B8-c6`, `f1-b5`, etc.
    Coordinate,

    /// This engine variant helps to return a `DescriptiveParser`,
    /// which parses moves in English descriptive notation.
    /// Example moves: `P-K4`, `NxN`, `QxRch`, `Q-KR4 mate`, `O-O`, etc.
    Descriptive,

    /// This engine variant helps to return a `ICCFParser`,
    /// which parses moves in ICCF notation.
    /// Example moves: `5254`, `5755`, `7163`, `2836`, `6125`, etc.
    ICCF,

    /// This engine variant helps to return a `LongAlgebraicParser`,
    /// which parses moves in long algebraic notation.
    /// Example moves: `e2e4`, `e7e5`, `d2d3`, `Bf8b4+`, `Bb5xc6`, etc.
    LongAlgebraic,

    /// This engine variant helps to return a `ReversibleAlgebraicParser`,
    /// which parses moves in reversible algebraic notation.
    /// Example moves: `e2-e4`, `e7-e5`, `Bb5xNc6`, `Bf8-b4#`, etc.
    ReversibleAlgebraic,

    /// This engine variant helps to return a `SmithParser`,
    /// which parses moves in Smith notation.
    /// Example moves: `e1g1c`, `b4c3n`, `b5c6n`, `d7c6b`, `e2e4`, etc.
    Smith,
}

/// Given a variant of `ParserEngine`, this function returns a
/// dynamic parser that implements the `Parse` trait.
///
/// Example:
///
/// ```
/// use chui::{Engine, Player, Color, parser::{self, ParserEngine}};
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
/// if let Ok(engine) = Engine::new(white, black, ParserEngine::Algebraic) {
///     println!("the move: {:?}", parser_engine.parse("P-K4", Color::White));
/// };
/// ```
pub fn new(parser: ParserEngine) -> Box<dyn Parser + Send + Sync> {
    match parser {
        ParserEngine::Algebraic => {
            algebraic::AlgebraicParser::new()
        },

        ParserEngine::ConciseReversible => {
            concise_reversible::ConciseReversibleParser::new()
        },

        ParserEngine::Coordinate => {
            coordinate::CoordinateParser::new()
        },

        ParserEngine::Descriptive => {
            descriptive::DescriptiveParser::new()
        },

        ParserEngine::ICCF => {
            iccf::ICCFParser::new()
        },

        ParserEngine::LongAlgebraic => {
            long_algebraic::LongAlgebraicParser::new()
        },

        ParserEngine::ReversibleAlgebraic => {
            reversible_algebraic::ReversibleAlgebraicParser::new()
        },

        ParserEngine::Smith => {
            smith::SmithParser::new()
        },
    }
}
