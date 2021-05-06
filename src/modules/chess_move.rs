//! Provides a struct for `Move` instances. Each move has properties
//! covering from and to coordinates, the piece being moved, the
//! original move text and interpreted move text. If the move is
//! invalid, `MoveType::Invalid` will occupy `self.move_type`.
//!
//! Also provides the `MoveType` enum.

use std::fmt;
//use std::ops;

use super::{Piece, Color};
use crate::ChuiError;

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
    pub piece: Option<Piece>,

    /// The parsed move text (e.g., "Pawn on e4 captures e5").
    pub move_text: String,

    /// The user's input move text (e.g., "Be5").
    pub input_move: String,

    /// Represents the type of move to be performed. A `Move`
    /// or a `Capture`. An `ErrorKind` is returned if the move is
    /// invalid.
    pub move_type: crate::Result<MoveType>,
}


/// Implement `Display` for `Move`. Displays the input move,
/// the move text, and the reason for an invalid move.
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}) {}",
            self.input_move, self.move_text
        )
    }
}

impl Move {
    /// Parse the input move.
    /// 
    /// TODO: implement a real parser.
    pub fn parse_move(the_move: &str, to_move: Color) -> Move {
        // Trim the whitespace from `the_move`, then check if
        // it contains a whitespace character. If so, the move
        // is invalid.
        let the_move = the_move.trim();

        if the_move == String::new() {
            return Move::invalid(the_move, "invalid empty input");
        }

        if the_move.contains(char::is_whitespace) {
            return Move::invalid(the_move, "contains whitespace");
        }

        // Just for development. A parser will be implemented soon.
        Move {
            from_coord: ('e', 2),
            to_coord: ('e', 4),
            from_index: (4, 1),
            to_index: (4, 3),
            piece: Some(Piece::Pawn(to_move)),
            move_text: String::from("Pawn from e2 to e4"),
            input_move: the_move.to_string(),
            move_type: Ok(MoveType::Move),
        }
    }

    /// Return an invalid move.
    pub fn invalid(the_move: &str, reason: &str) -> Move {
        Move {
            from_coord: ('-', 0),
            to_coord: ('-', 0),
            from_index: (8, 8),
            to_index: (8, 8),
            piece: None,
            move_text: String::from("invalid move"),
            input_move: the_move.to_string(),
            move_type: Err(ChuiError::InvalidMove(reason.to_string())), 
        }
    }
}
