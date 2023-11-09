//! Chui Trait Definitions

use crate::prelude::*;

/// Implement this trait to define the `parse()` method on a parser.
/// Any struct implementing this trait should parse a chess move
/// in an expected notation and return a `ChessMove` object, representing
/// the validty or invalidity of the requested move for the given
/// chessboard.
///
/// Example:
///
/// ```
/// use chui_core::prelude::*;
///
/// pub struct MyParser;
///
/// impl Parser for MyParser {
///     fn parse(&mut self, _the_move: String, _color: Color) -> ChuiResult<Move> {
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
///
///     fn generate_move_from_board_coordinates(
///         &self,
///         game: &Game,
///         from_coord: Coord,
///         to_coord: Coord,
///     ) -> ChuiResult<String> {
///         Ok("E.g., `Ba1`".to_string())
///     }
/// }
/// ```
pub trait Parser: Send + Sync {
    /// Parse the chess move, return `Ok(Move)` on success,
    /// `ChuiError::InvalidMove(reason)` on failure.
    ///
    /// # Errors
    ///
    /// * Errors when the parser cannot parse a move.
    fn parse(&mut self, move_string: String, to_move: Color) -> ChuiResult<ChessMove>;

    /// The name of the parser. Used in help messages and debug.
    fn name(&self) -> String;

    /// Example inputs.
    fn eg(&self) -> String;

    /// Generate move from board Coordinates into this parser's notation.
    ///
    /// # Errors
    ///
    /// * Errors when the parser cannot generate a move from the board Coordinates.
    fn generate_move_from_board_coordinates(
        &self,
        game: &Game,
        from_coord: Coord,
        to_coord: Coord,
    ) -> ChuiResult<String>;

    /// Trim the whitespace from `the_move` and check to see that
    /// the move doesn't contain any whitespace after the trim.
    ///
    /// # Errors
    ///
    /// * Errors when the input move is empty.
    /// * Errors when the input move contains whitespace.
    fn trim_and_check_whitespace(&self, move_string: &str) -> ChuiResult<String> {
        let the_move: String = move_string.trim().to_string();

        if the_move.eq("") {
            self.invalid_input("Input move cannot be empty")?;
        }

        if the_move.contains(char::is_whitespace) {
            self.invalid_input("Input move contains whitespace")?;
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

    /// Match the given index (`u8`) to its file (`char`).
    fn match_index_to_file(&self, index: u8) -> Option<char> {
        match index {
            0 => Some('a'),
            1 => Some('b'),
            2 => Some('c'),
            3 => Some('d'),
            4 => Some('e'),
            5 => Some('f'),
            6 => Some('g'),
            7 => Some('h'),
            _ => None,
        }
    }

    /// Match the given index (`u8`) to its rank (`char`).
    fn match_index_to_rank(&self, index: u8) -> Option<char> {
        match index {
            0 => Some('1'),
            1 => Some('2'),
            2 => Some('3'),
            3 => Some('4'),
            4 => Some('5'),
            5 => Some('6'),
            6 => Some('7'),
            7 => Some('8'),
            _ => None,
        }
    }

    /// Return a `ChuiError` indicating Invalid Input.
    ///
    /// # Errors
    ///
    /// * Errors all the time.
    fn invalid_input(&self, reason: &str) -> ChuiResult<()> {
        Err(ChuiError::InvalidInput(reason.to_string()))
    }
}
