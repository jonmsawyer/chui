//! Provides a struct for `Move` instances. Each move has properties
//! covering from and to coordinates, the piece being moved, the
//! original move text and interpreted move text. If the move is
//! invalid, `MoveType::Invalid` will occupy `self.move_type`.
//!
//! Also provides the `MoveType` enum.

use std::fmt;

use super::Piece;

/// Represents the type of move to be performed. If the move
/// is a simple move, then `Move` is represented. If the move
/// is a piece or pawn capture, `Capture` is represented. If the
/// move is invalid, `Invalid` is represented.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveType {
    PawnMove,

    PawnCapture,

    PieceMove,

    PieceCapture,

    Castle,
}

/// Represents a chess move.
#[derive(Debug, PartialEq)]
pub struct Move {
    /// Represents a move's "from" coordinate (e.g., ('a', 1)).
    pub from_coord: (char, u8),

    /// Represents a move's "to" coordinate (e.g., ('b' 8)).
    pub to_coord: (char, u8),

    /// Represents a move's "from" index (e.g., (0, 0) \[a1\]).
    pub from_index: (u8, u8),

    /// Represents a move's "to" index (e.g., (1, 7) \[b8\]).
    pub to_index: (u8, u8),

    /// The chess piece to move
    pub piece: Option<Piece>,

    /// In check.
    pub check: bool,

    /// In checkmate.
    pub check_mate: bool,

    /// Is move promotion?
    pub promotion: bool,

    /// Promotion piece
    pub promotion_piece: Option<Piece>,

    /// Is castling move
    pub is_castling: bool,

    /// Is castling king side
    pub is_castling_king: bool,

    /// Is castling quuen side
    pub is_castling_queen: bool,

    /// The parsed move text (e.g., "Pawn on e4 captures e5").
    pub move_text: String,

    /// The user's input move text (e.g., "Be5").
    pub input_move: String,

    /// Represents the type of move to be performed. A `Move`
    /// or a `Capture`. An `ErrorKind` is returned if the move is
    /// invalid.
    pub move_type: Option<MoveType>,
}

/// Displays the input move and move text.
impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = format!("{{
    from_coord: {:?},
    to_coord: {:?},
    from_index: {:?},
    to_index: {:?},
    piece: {:?},
    check: {:?},
    check_mate: {:?},
    promotion: {:?},
    promotion_piece: {:?},
    is_castling: {:?},
    is_castling_king: {:?},
    is_castling_queen: {:?},
    move_text: {:?},
    input_move: {:?},
    move_type: {:?}
}}",
            self.from_coord,
            self.to_coord,
            self.from_index,
            self.to_index,
            self.piece,
            self.check,
            self.check_mate,
            self.promotion,
            self.promotion_piece,
            self.is_castling,
            self.is_castling_king,
            self.is_castling_queen,
            self.move_text,
            self.input_move,
            self.move_type,
        );

        write!(f, "Move {}", output)
    }
}

impl Default for Move {
    fn default() -> Self {
        Self::new()
    }
}

impl Move {
    /// Return a new default `Move`.
    pub fn new() -> Move {
        Move {
            from_coord: ('-', 8),
            to_coord: ('-', 8),
            from_index: (8, 8),
            to_index: (8, 8),
            piece: None,
            check:false,
            check_mate: false,
            promotion: false,
            promotion_piece: None,
            is_castling: false,
            is_castling_king: false,
            is_castling_queen: false,
            move_text: String::new(),
            input_move: String::new(),
            move_type: None,
        }
    }
}
