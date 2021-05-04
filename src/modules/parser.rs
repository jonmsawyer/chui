#![allow(clippy::upper_case_acronyms)]

use super::{Move, Piece};

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
/// use chui::{Move, Piece, parser::Parser};
/// 
/// pub struct MyParser;
/// 
/// impl Parser for MyParser {
///     fn parse(&self, the_move: &str, _board: &[[Option<Piece>; 8]; 8]) -> Move {
///         Move::invalid(the_move, "Error: MyParser not implemented.")
///     }
/// }
/// ```
pub trait Parser {
    fn parse(&self, the_move: &str, board: &[[Option<Piece>; 8]; 8]) -> Move;
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
/// let engine = Engine::new(white, black);
/// let parser = parser::new(ParserEngine::Descriptive);
/// 
/// println!("the move: {:?}", parser.parse("P-K4", &engine.board));
/// ```
pub fn new(parser: ParserEngine) -> Box<dyn Parser> {
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
