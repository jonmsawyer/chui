//! ICCF notation module.

#![allow(clippy::new_ret_no_self)]

use crate::prelude::*;

/// A parser that will parse ICCF chess notation.
/// Example moves: `5254`, `5755`, `7163`, `2836`, `6125`, etc.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ICCFParser {
    /// The color to move.
    pub to_move: Color,
}

impl Parser for ICCFParser {
    /// Parse the chess move, return `Ok(Move)` on success,
    /// `ChuiError::InvalidMove(reason)` on failure.
    fn parse(&mut self, move_string: String, to_move: Color) -> ChuiResult<ChessMove> {
        self.to_move = to_move;
        let char_list: Vec<char> = vec!['1', '2', '3', '4', '5', '6', '7', '8'];
        let mut the_move: String = self.trim_and_check_whitespace(&move_string)?;
        the_move.retain(|c: char| char_list.contains(&c));
        if the_move.len() != 4 {
            self.invalid_input(
                format!(
                "{} is an invalid move: invalid length (move length is {} but it needs to be 4)",
                the_move,
                the_move.len(),
            )
                .as_str(),
            )?;
        }
        println!("The move: {:?}", the_move);
        let from_coord: Coord = Coord::try_from((
            the_move.remove(0).to_digit(10).unwrap() - 1,
            the_move.remove(0).to_digit(10).unwrap() - 1,
        ))?;
        let to_coord: Coord = Coord::try_from((
            the_move.remove(0).to_digit(10).unwrap() - 1,
            the_move.remove(0).to_digit(10).unwrap() - 1,
        ))?;
        from_coord.validate_possible_move(to_coord)?;
        let chess_move: ChessMove = ChessMove {
            to_move,
            from_coord: Some(from_coord),
            to_coord: Some(to_coord),
            input_move: move_string,
            ..ChessMove::default()
        };
        println!("The move: {:?}", chess_move);
        Ok(chess_move)
    }

    fn name(&self) -> String {
        "ICCF Parser".to_string()
    }

    fn eg(&self) -> String {
        format!(
            "Examples for {}: `5254`, `5755`, `7163`, `2836`, `6125`, etc.",
            self.name()
        )
    }

    /// Return a String representing the move from board Coordinates to this
    /// parser's notation.
    fn generate_move_from_board_coordinates(
        &self,
        _game: &Game,
        _from_coord: Coord,
        _to_coord: Coord,
    ) -> ChuiResult<String> {
        Err(ChuiError::NotImplemented(
            "generate_move_from_board_coordinates".to_string(),
        ))
    }
}

impl ICCFParser {
    /// Return a new dynamic parser that implements the `Parser` trait.
    pub fn new(to_move: Color) -> Box<ICCFParser> {
        Box::new(ICCFParser { to_move })
    }
}
