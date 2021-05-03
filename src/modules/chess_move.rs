//! File: `chess_move.rs`
//!
//! Module: `chess_move`
//!
//! Provides a struct for `Move` instances. Each move has properties
//! covering from and to coordinates, the piece being moved, the
//! original move text and interpreted move text. If the move is
//! invalid, `MoveType::Invalid` will occupy `self.move_type`.
//!
//! Also provides the `MoveType` and `MoveState` enums.

use std::fmt;
//use std::ops;

use super::{Piece, PieceColor};

/// Represents the state of the move being performed. `Valid` if
/// the move is valid, otherwise `Invalid`.
#[derive(Debug)]
pub enum MoveState {
    /// Represents the valid move state.
    Valid,

    /// Represents the invalid move state.
    Invalid,
}

/// Represents the type of move to be performed. If the move
/// is a simple move, then `Move` is represented. If the move
/// is a piece or pawn capture, `Capture` is represented. If the
/// move is invalid, `Invalid` is represented.
#[derive(Debug)]
pub enum MoveType {
    /// Represents that the desired move is just a move, not
    /// a capture.
    Move,

    /// Represents that the desired move is a capture, not just
    /// a move.
    Capture,

    /// Represents that the desired move is invalid.
    Invalid,
}

/// Represents a chess move.
#[derive(Debug)]
pub struct Move {
    /// Represents a move's "from" coordinate (e.g., ('a', 1)).
    pub from_coord: (char, u32),

    /// Represents a move's "to" coordinate (e.g., ('b' 8)).
    pub to_coord: (char, u32),

    /// Represents a move's "from" index (e.g., (0, 0) \[a1\]).
    pub from_index: (u32, u32),

    /// Represents a move's "to" index (e.g., (1, 7) \[b8\]).
    pub to_index: (u32, u32),

    /// The chess piece to move
    pub piece: Piece,

    /// The parsed move text (e.g., "Pawn on e4 captures e5").
    pub move_text: String,

    /// The user's input move text (e.g., "Be5").
    pub input_move: String,

    /// The move's `MoveState`. One of `Move`, `Capture`, or `Invalid`.
    pub move_state: MoveState,

    /// The move's `MoveType`. One of `Valid` or `Invalid`.
    pub move_type: MoveType,

    /// The reason if `MoveState` or `MoveType` is `Invalid`.
    pub reason: String,
}


/// Implement `Display` for `Move`. Displays the input move,
/// the move text, and the reason for an invalid move.
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}) {} ({})",
            self.input_move, self.move_text, self.reason
        )
    }
}

impl Move {
    /// Parse a coordinate move (e.g., "a1-a2").
    /// 
    /// TODO: implement a real parser.
    pub fn parse_square_to_square_move(the_move: &str) -> Move {
        let move_tokens: Vec<&str> = the_move.split('-').collect();

        if move_tokens.len() != 2 {
            return Move::invalid(the_move, "num tokens != 2 [split on '-']");
        }

        //
        // Part One
        //

        let part_one = move_tokens[0]; // must be valid `{a-h}{1-8}`

        if part_one.len() != 2 {
            return Move::invalid(
                the_move,
                &format!("len of first part != 2 [{}]", part_one)
            );
        }

        let part_one_chars: Vec<char> = part_one.chars().collect();

        let mut round = 1;
        for c in part_one_chars.iter() {
            if round == 1 && !('a'..='h').contains(&c) {
                return Move::invalid(
                    the_move,
                    &format!(
                        "first part, {}, not in [a, b, c, d, e, f, g, h]",
                        c
                    ),
                );
            } else if round == 2 && !('1'..='8').contains(&c) {
                return Move::invalid(
                    the_move,
                    &format!(
                        "second part, {}, not in [1, 2, 3, 4, 5, 6, 7, 8]",
                        c
                    ),
                );
            }
            round += 1;
        }

        //
        // Part Two
        //

        let part_two = move_tokens[1]; // must be valid {a-h}{1-8}

        if part_two.len() != 2 {
            return Move::invalid(the_move, &format!("len of second part != 2 [{}]", part_two));
        }

        let part_two_chars: Vec<char> = part_two.chars().collect();

        let mut round = 1;
        for c in part_two_chars.iter() {
            if round == 1 && !('a'..='h').contains(&c) {
                return Move::invalid(
                    the_move,
                    &format!("first part, {}, not in [a, b, c, d, e, f, g, h]", c),
                );
            } else if round == 2 && !('1'..='8').contains(&c) {
                return Move::invalid(
                    the_move,
                    &format!("second part, {}, not in [1, 2, 3, 4, 5, 6, 7, 8]", c),
                );
            }
            round += 1;
        }

        Move {
            from_coord: (part_one_chars[0], part_one_chars[1].to_digit(10).unwrap()),
            to_coord: (part_two_chars[0], part_two_chars[1].to_digit(10).unwrap()),
            from_index: (0, 0),
            to_index: (0, 0),
            piece: Piece::Pawn(PieceColor::White),
            move_text: the_move.to_string(),
            input_move: the_move.to_string(),
            move_state: MoveState::Valid,
            move_type: MoveType::Move,
            reason: String::from("foo"),
        }
    }

    /// Parse a piece capture move.
    /// 
    /// TODO: implement a real parser.
    pub fn parse_piece_capture_move(the_move: &str) -> Move {
        let move_tokens: Vec<&str> = the_move.split('x').collect();
        if move_tokens.len() != 2 {
            return Move::invalid(the_move, "num tokens != 2 [split on 'x']");
        }
        Move::invalid(the_move, "not implemented")
    }

    /// Parse the input move.
    /// 
    /// TODO: implement a real parser.
    pub fn parse_move(the_move: &str, to_move: PieceColor) -> Move {
        // Trim the whitespace from `the_move`, then check if
        // it contains a whitespace character. If so, the move
        // is invalid.
        let the_move = the_move.trim();

        if the_move.contains(char::is_whitespace) {
            return Move::invalid(the_move, "contains whitespace");
        }

        // Move is similar to `square-to-square`, now must expect
        // `{col}{row}-{col}{row}{{=}RNBQ|+|++|#}` format.
        if the_move.contains('-') {
            return Move::parse_square_to_square_move(the_move);
        }

        // Move is similar to `(square|piece)-takes-square`,
        // now must expect `{col}{row}x{col}{row}` or
        // `{piece}x{col}{row}{{=}RNBQ|+|++|#}`.
        else if the_move.contains('x') {
            return Move::parse_piece_capture_move(the_move);
        }

        Move {
            from_coord: ('a', 1),
            to_coord: ('a', 8),
            from_index: (7, 0),
            to_index: (0, 0),
            piece: Piece::Rook(to_move),
            move_text: String::from("Pawn from e2 to e4"),
            input_move: String::from(the_move),
            move_state: MoveState::Valid,
            move_type: MoveType::Move,
            reason: String::new(),
        }
    }

    /// Return an invalid move.
    pub fn invalid(the_move: &str, reason: &str) -> Move {
        Move {
            from_coord: ('-', 0),
            to_coord: ('-', 0),
            from_index: (8, 8),
            to_index: (8, 8),
            piece: Piece::None,
            move_text: String::from("invalid move"),
            input_move: the_move.to_string(),
            move_state: MoveState::Invalid,
            move_type: MoveType::Invalid,
            reason: reason.to_string(),
        }
    }
}
