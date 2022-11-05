//! Provides a struct for `Move` instances. Each move has properties
//! covering from and to coordinates, the piece being moved, the
//! original move text and interpreted move text. If the move is
//! invalid, `MoveType::Invalid` will occupy `self.move_type`.
//!
//! Also provides the `MoveType` enum.

use std::fmt;

use crate::{ChuiError, ChuiResult};

use super::{Color, Piece, PieceKind};

/// Represents the type of move to be performed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveType {
    PawnMove,

    PawnCapture,

    PieceMove,

    PieceCapture,

    Castle,
}

/// Represents a chess move.
#[derive(PartialEq, Clone)]
pub struct Move {
    /// Represents a move's "from" coordinate (e.g., `('a', 1)`).
    pub from_coord: (char, u8),

    /// Represents a move's "to" coordinate (e.g., `('b' 8)`).
    pub to_coord: (char, u8),

    /// Represents a move's "from" index (e.g., `(0, 0)` \[a1\]).
    pub from_index: (u8, u8),

    /// Represents a move's "to" index (e.g., `(1, 7)` \[b8\]).
    pub to_index: (u8, u8),

    /// The chess piece to move.
    pub piece: Option<Piece>,

    /// In check?
    pub check: bool,

    /// In check mate?
    pub check_mate: bool,

    /// Is move promotion?
    pub promotion: bool,

    /// Promotion piece.
    pub promotion_piece: Option<Piece>,

    /// Is castling move?
    pub is_castling: bool,

    /// Is castling king side?
    pub is_castling_king: bool,

    /// Is castling queen side?
    pub is_castling_queen: bool,

    /// The user's input move text (e.g., `Be5`).
    pub input_move: String,

    /// Represents the type of move to be performed. A `Some(Move)`
    /// or a `Some(Capture)`. A `None` if the move is invalid.
    pub move_type: Option<MoveType>,
}

/// Displays the input move and move text.
impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = format!(
            "{{
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

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.input_move)
    }
}

impl Move {
    /// Return a new default `Move`.
    pub fn new() -> Move {
        Move {
            from_coord: ('-', 9),
            to_coord: ('-', 9),
            from_index: (8, 8),
            to_index: (8, 8),
            piece: None,
            check: false,
            check_mate: false,
            promotion: false,
            promotion_piece: None,
            is_castling: false,
            is_castling_king: false,
            is_castling_queen: false,
            input_move: String::new(),
            move_type: None,
        }
    }

    //
    // boolean checks
    //

    /// Is pawn move?
    pub fn is_pawn_move(&self) -> bool {
        matches!(self.move_type, Some(MoveType::PawnMove))
    }

    /// Is pawn capture?
    pub fn is_pawn_capture(&self) -> bool {
        matches!(self.move_type, Some(MoveType::PawnCapture))
    }

    /// Is piece move?
    pub fn is_piece_move(&self) -> bool {
        matches!(self.move_type, Some(MoveType::PieceMove))
    }

    /// Is piece capture?
    pub fn is_piece_capture(&self) -> bool {
        matches!(self.move_type, Some(MoveType::PieceCapture))
    }

    /// Is castle?
    pub fn is_castle(&self) -> bool {
        matches!(self.move_type, Some(MoveType::Castle))
    }

    /// Is check?
    pub fn is_check(&self) -> bool {
        self.check
    }

    /// Is castling?
    pub fn is_castling(&self) -> bool {
        self.is_castling && (self.is_castling_king || self.is_castling_queen)
    }

    /// Is castling king side?
    pub fn is_castling_king(&self) -> bool {
        self.is_castling && self.is_castling_king && !self.is_castling_queen
    }

    /// Is castling queen side?
    pub fn is_castling_queen(&self) -> bool {
        self.is_castling && !self.is_castling_king && self.is_castling_queen
    }

    //
    // Getters
    //

    /// Get moving piece.
    pub fn get_piece(&self) -> Option<Piece> {
        self.piece
    }

    //
    // Setters
    //

    /// Set the input move.
    pub fn set_input_move(&mut self, the_move: String) {
        self.input_move = the_move;
    }

    /// Set the moving piece.
    pub fn set_piece(&mut self, piece: Piece) {
        self.piece = Some(piece)
    }

    /// Set the move type.
    pub fn set_move_type(&mut self, move_type: MoveType) {
        self.move_type = Some(move_type)
    }

    /// Set the color of the pieces (after they have already been
    /// parsed).
    pub fn set_color(&mut self, color: Color) {
        if self.piece.is_some() {
            let piece = self.piece.as_mut().unwrap();
            piece.set_color(color);
            self.piece = Some(*piece);
        }

        if self.promotion_piece.is_some() {
            let piece = self.promotion_piece.as_mut().unwrap();
            piece.set_color(color);
            self.promotion_piece = Some(*piece);
        }
    }

    /// Set castling king.
    pub fn set_castling_king(&mut self) {
        self.is_castling = true;
        self.is_castling_king = true;
        self.is_castling_queen = false;
        self.set_piece(Piece::new(PieceKind::King, Color::White));
        self.set_move_type(MoveType::Castle);
    }

    /// Set castling queen.
    pub fn set_castling_queen(&mut self) {
        self.is_castling = true;
        self.is_castling_king = false;
        self.is_castling_queen = true;
        self.set_piece(Piece::new(PieceKind::King, Color::White));
        self.set_move_type(MoveType::Castle);
    }

    /// Set pawn move.
    pub fn set_pawn_move(&mut self) {
        self.set_piece(Piece::new(PieceKind::Pawn, Color::White));
        self.set_move_type(MoveType::PawnMove);
    }

    /// Set piece move.
    pub fn set_piece_move(&mut self, piece: Piece) {
        self.set_piece(piece);
        self.set_move_type(MoveType::PieceMove);
    }

    /// If there is a pawn move or piece move registered, set either
    /// one of those to a capture move. If the move type is invalid,
    /// return an error.
    pub fn set_capture(&mut self) -> ChuiResult<()> {
        self.move_type = match self.move_type {
            Some(MoveType::PawnMove) => Some(MoveType::PawnCapture),
            Some(MoveType::PieceMove) => Some(MoveType::PieceCapture),
            move_type => {
                return Err(ChuiError::InvalidMove(format!(
                    "`{:?}` move type is invalid for capture",
                    move_type
                )));
            }
        };

        Ok(())
    }

    /// Set check.
    pub fn set_check(&mut self) {
        self.check = true;
        self.check_mate = false;
    }

    /// Set check mate.
    pub fn set_check_mate(&mut self) {
        self.check = false;
        self.check_mate = true;
    }

    /// Set promotion.
    pub fn set_promotion(&mut self) {
        self.promotion = true;
    }

    /// Set promotion piece.
    pub fn set_promotion_piece(&mut self, piece: Piece) {
        self.set_promotion();
        self.promotion_piece = Some(piece);
    }

    /// Set the `to_coord` file.
    pub fn set_to_coord_file(&mut self, file: char) {
        self.from_coord = (self.to_coord.0, self.from_coord.1);
        self.to_coord = (file, self.to_coord.1);
    }

    /// Set the `to_coord` rank.
    pub fn set_to_coord_rank(&mut self, rank: u8) {
        self.from_coord = (self.from_coord.0, self.to_coord.1);
        self.to_coord = (self.to_coord.0, rank);
    }

    /// Set the `to_index` file.
    pub fn set_to_index_file(&mut self, file: u8) {
        self.from_index = (self.to_index.0, self.to_index.1);
        self.to_index = (file, self.to_index.1);
    }

    /// Set the `to_index` rank.
    pub fn set_to_index_rank(&mut self, rank: u8) {
        self.from_index = (self.from_index.0, self.to_index.1);
        self.to_index = (self.to_index.0, rank);
    }

    //
    // Updaters
    //

    /// Return the verbose move text.
    pub fn get_move_text(&self) -> String {
        if self.piece.is_none() || self.move_type.is_none() {
            return String::new();
        }

        // E.g., "White King"
        let mut move_text = self.piece.unwrap().get_text();

        let mut is_capture = false;

        // Is moving, capturing, or castling?
        let move_verb = match self.move_type {
            Some(MoveType::PawnMove) | Some(MoveType::PieceMove) => "moves",
            Some(MoveType::PawnCapture) | Some(MoveType::PieceCapture) => {
                is_capture = true;
                "captures"
            }
            Some(MoveType::Castle) => "castles",
            _ => "", // should never match
        };

        move_text = format!("{} {}", move_text, move_verb);

        // Is castling King or Queen side?
        if self.is_castling && self.is_castling_king {
            return format!("{} King side", move_text);
        } else if self.is_castling && self.is_castling_queen {
            return format!("{} Queen side", move_text);
        }

        let (from_file, from_rank) = self.from_coord;

        let mut is_from = false;
        if from_file != '-' || from_rank <= 8 {
            is_from = true;
            move_text = format!("{} from ", move_text);
        }

        if from_file != '-' {
            move_text = format!("{}{}", move_text, from_file);
        }

        if from_rank <= 8 {
            move_text = format!("{}{}", move_text, from_rank);
        }

        let (to_file, to_rank) = self.to_coord;

        if to_file != '-' || from_rank <= 8 {
            if is_capture && !is_from {
                move_text = format!("{} ", move_text);
            } else {
                move_text = format!("{} to ", move_text);
            }
        }

        if to_file != '-' {
            move_text = format!("{}{}", move_text, to_file);
        }

        if to_rank <= 8 {
            move_text = format!("{}{}", move_text, to_rank);
        }

        if self.promotion {
            if let Some(piece) = self.promotion_piece {
                move_text = format!("{} promotes to {}", move_text, piece.get_text());
            }
        }

        if self.check {
            move_text = format!("{} check", move_text);
        } else if self.check_mate {
            move_text = format!("{} check mate", move_text);
        }

        move_text
    }
}
