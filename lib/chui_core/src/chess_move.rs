//! Provides a struct for `ChessMove` instances. Each move has properties
//! covering from and to `Coordinate`s, the piece being moved, the
//! original move text and interpreted move text.
//!
//! Also provides the `Casting`, `Check`, and `MoveType` enums.

use std::fmt;

use crate::prelude::{coord::*, *};

/// Represents the type of castling to be performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Castling {
    /// Castling King side.
    King,

    /// Castling Queen side.
    Queen,
}

/// Represents the type of check that is flagged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Check {
    /// Check.
    Check,

    /// Check mate.
    Mate,
}

/// Represents the type of move to be performed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MoveType {
    /// This move is a pawn move.
    PawnMove,

    /// This move is a pawn capture.
    PawnCapture,

    /// This move is a piece move (not pawn move).
    PieceMove,

    /// This move is a piece capture (not pawn capture).
    PieceCapture,

    /// This move is a castling move, king side or queen side.
    Castle,
}

/// Represents a chess move.
#[derive(PartialEq, Eq, Clone)]
pub struct ChessMove {
    /// The color to move.
    pub to_move: Color,

    /// Represents a `ChessMove`'s "from" Coordinate (e.g., `('a', 1)`).
    pub from_coord: Option<Coord>,

    /// Represents the from move's file index (0..=7).
    pub from_coord_file: Option<NonMaxU8>,

    /// Represents the from move's rank index (0..=7).
    pub from_coord_rank: Option<NonMaxU8>,

    /// The chess piece to move.
    pub from_piece: Option<Piece>,

    /// Represents a `ChessMove`'s "to" Coordinate (e.g., `('b', 8)`).
    pub to_coord: Option<Coord>,

    /// Represents the to move's file index (0..=7).
    pub to_coord_file: Option<NonMaxU8>,

    /// Represents the to move's rank index (0..=7).
    pub to_coord_rank: Option<NonMaxU8>,

    /// The chess piece to capture.
    pub to_piece: Option<Piece>,

    /// Check or check mate.
    pub check: Option<Check>,

    /// Promotion piece.
    pub promotion: Option<Piece>,

    /// Is castling move?
    pub castling: Option<Castling>,

    /// En passant coordinate.
    pub en_passant: Option<Coord>,

    /// Represents the type of move to be performed.
    pub move_type: Option<MoveType>,

    /// The user's input move text (e.g., `Be5`).
    pub input_move: String,

    /// Have the attributes in this chess move been modified by a `Parser`?
    /// TODO: Is this necessary?
    pub is_parsed: bool,

    /// Any validation errors.
    pub validation_errors: Vec<ChuiError>,
}

/// Displays the input move and move text.
impl fmt::Debug for ChessMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format validation errors.
        let mut errors: String = String::from("[\n");
        if !self.validation_errors.is_empty() {
            for error in self.validation_errors.iter() {
                errors.push_str(format!("        {:?},\n", error).as_str());
            }
            errors.push_str("    ],");
        } else {
            errors = String::from("[],");
        }
        // Format `from_coord` and `to_coord`.
        let from_coord: String = if self.from_coord.is_some() {
            format!("{}", self.from_coord.unwrap())
        } else {
            "<None>".to_string()
        };
        let to_coord: String = if self.to_coord.is_some() {
            format!("{}", self.to_coord.unwrap())
        } else {
            "<None>".to_string()
        };
        // Format output.
        let output: String = format!(
            "{{
    to_move: {:?},
    from_coord: {} ({:?}),
    from_coord_file: {:?},
    from_coord_rank: {:?},
    from_piece: {:?},
    to_coord: {} ({:?}),
    to_coord_file: {:?},
    to_coord_rank: {:?},
    to_piece: {:?},
    check: {:?},
    promotion: {:?},
    castling: {:?},
    move_type: {:?},
    en_passant: {:?},
    input_move: {:?},
    parsed: {:?},
    validation_errors: {},
}}",
            self.to_move,
            from_coord,
            self.from_coord,
            self.from_coord_file,
            self.from_coord_rank,
            self.from_piece,
            to_coord,
            self.to_coord,
            self.to_coord_file,
            self.to_coord_rank,
            self.to_piece,
            self.check,
            self.promotion,
            self.castling,
            self.en_passant,
            self.move_type,
            self.input_move,
            self.is_parsed,
            errors,
        );

        write!(f, "Move {}", output)
    }
}

impl Default for ChessMove {
    fn default() -> Self {
        ChessMove {
            to_move: Color::White,
            from_coord: None,
            from_coord_file: None,
            from_coord_rank: None,
            from_piece: None,
            to_coord: None,
            to_coord_file: None,
            to_coord_rank: None,
            to_piece: None,
            check: None,
            promotion: None,
            castling: None,
            en_passant: None,
            input_move: String::new(),
            move_type: None,
            is_parsed: false,
            validation_errors: Vec::<ChuiError>::new(),
        }
    }
}

impl fmt::Display for ChessMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.input_move)
    }
}

impl ChessMove {
    /// Return a new default `ChessMove`.
    pub fn new(to_move: Color) -> ChessMove {
        ChessMove {
            to_move,
            ..ChessMove::default()
        }
    }

    //
    // Boolean checks.
    //

    /// Is it White to move?
    pub const fn is_white_move(&self) -> bool {
        matches!(self.to_move, Color::White)
    }

    /// Is it Black to move?
    pub const fn is_black_move(&self) -> bool {
        matches!(self.to_move, Color::Black)
    }

    /// Is pawn move?
    pub const fn is_pawn_move(&self) -> bool {
        if let Some(move_type) = self.move_type {
            matches!(move_type, MoveType::PawnMove)
        } else {
            false
        }
    }

    /// Is pawn capture?
    pub const fn is_pawn_capture(&self) -> bool {
        if let Some(move_type) = self.move_type {
            matches!(move_type, MoveType::PawnCapture)
        } else {
            false
        }
    }

    /// Is piece move?
    pub const fn is_piece_move(&self) -> bool {
        if let Some(move_type) = self.move_type {
            matches!(move_type, MoveType::PieceMove)
        } else {
            false
        }
    }

    /// Is piece capture?
    pub const fn is_piece_capture(&self) -> bool {
        if let Some(move_type) = self.move_type {
            matches!(move_type, MoveType::PieceCapture)
        } else {
            false
        }
    }

    /// Is castle?
    pub const fn is_castle(&self) -> bool {
        self.castling.is_some()
    }

    /// Is check?
    pub const fn is_check(&self) -> bool {
        if let Some(check) = self.check {
            matches!(check, Check::Check)
        } else {
            false
        }
    }

    /// Is check mate?
    pub const fn is_check_mate(&self) -> bool {
        if let Some(check) = self.check {
            matches!(check, Check::Mate)
        } else {
            false
        }
    }

    /// Is castling?
    pub const fn is_castling(&self) -> bool {
        self.castling.is_some()
    }

    /// Is castling king side?
    pub const fn is_castling_king(&self) -> bool {
        if let Some(castling) = self.castling {
            matches!(castling, Castling::King)
        } else {
            false
        }
    }

    /// Is castling queen side?
    pub const fn is_castling_queen(&self) -> bool {
        if let Some(castling) = self.castling {
            matches!(castling, Castling::Queen)
        } else {
            false
        }
    }

    //
    // Getters.
    //

    /// Get moving piece.
    pub const fn get_piece(&self) -> Option<Piece> {
        self.from_piece
    }

    /// Return the verbose move text.
    ///
    /// # Panics
    ///
    /// Panics when `self.piece` is None after checking that it's Some.
    ///
    /// TODO: Review and refactor.
    pub fn get_move_text(&self) -> String {
        if self.from_piece.is_none() || self.move_type.is_none() {
            return String::new();
        }

        // E.g., "White King"
        let mut move_text = self.from_piece.expect("Piece cannot be None.").get_text();

        let mut is_capture = false;

        // Is moving, capturing, or castling?
        let move_verb = match self.move_type {
            Some(MoveType::PawnMove | MoveType::PieceMove) => "moves to",
            Some(MoveType::PawnCapture | MoveType::PieceCapture) => {
                is_capture = true;
                "captures on"
            }
            Some(MoveType::Castle) => "castles",
            None => "<no move verb>",
        };

        move_text = format!("{} {}", move_text, move_verb);

        // Is castling King or Queen side?
        if self.is_castling_king() {
            return format!("{} King side", move_text);
        } else if self.is_castling_queen() {
            return format!("{} Queen side", move_text);
        }

        let (from_file, from_rank) = self
            .from_coord
            .map_or(('-', 9), |from_coord| from_coord.to_char_u8_coord());

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

        let (to_file, to_rank) = self
            .to_coord
            .map_or(('-', 9), |to_coord| to_coord.to_char_u8_coord());

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

        if let Some(piece) = self.promotion {
            move_text = format!("{} promotes to {}", move_text, piece.get_text());
        }

        if self.is_check() {
            move_text = format!("{} check", move_text);
        } else if self.is_check_mate() {
            move_text = format!("{} check mate", move_text);
        }

        move_text
    }

    #[allow(clippy::unused_self)]
    /// Match the given file (`char`) to its index (`u8`).
    const fn match_file_to_index(&self, file: char) -> Option<u8> {
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

    #[allow(clippy::unused_self)]
    /// Match the given rank (`char`) to its index (`u8`).
    const fn match_rank_to_index(&self, rank: char) -> Option<u8> {
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

    //
    // Setters.
    //

    /// Set the input move.
    pub fn set_input_move(&mut self, the_move: String) {
        self.input_move = the_move;
    }

    /// Set the moving piece.
    pub fn set_piece(&mut self, piece: Piece) {
        self.from_piece = Some(piece);
        match piece.get_kind() {
            PieceKind::Pawn => self.set_move_type(MoveType::PawnMove),
            _ => self.set_move_type(MoveType::PieceMove),
        }
    }

    /// Set the move type.
    pub fn set_move_type(&mut self, move_type: MoveType) {
        self.move_type = Some(move_type);
    }

    /// Set the color of the pieces (after they have already been
    /// parsed).
    ///
    /// # Panics
    ///
    /// Panics when `self.piece` is None after checking if it's Some.
    ///
    /// TODO: Review.
    pub fn set_color(&mut self, color: Color) {
        if self.from_piece.is_some() {
            let piece: &mut Piece = self.from_piece.as_mut().expect("Piece cannot be None.");
            piece.set_color(color);
            self.from_piece = Some(*piece);
        }

        if self.promotion.is_some() {
            let piece: &mut Piece = self
                .promotion
                .as_mut()
                .expect("Promotion piece cannot be None.");
            piece.set_color(color);
            self.promotion = Some(*piece);
        }
    }

    /// Set castling king.
    ///
    /// # Panics
    ///
    /// Panics if a `Piece` (White King) could not be constructed and placed on the E1
    /// square. This functiion should never panic in this case.
    ///
    /// TODO: Do we care about the actual King that is castling?
    pub fn set_castling_king(&mut self) {
        self.castling = Some(Castling::King);
        self.set_piece(Piece::white_king(E1).unwrap());
        self.set_move_type(MoveType::Castle);
    }

    /// Set castling queen.
    ///
    /// # Panics
    ///
    /// Panics if a `Piece` (White King) could not be constructed and placed on the E1
    /// square. This functiion should never panic in this case.
    ///
    /// TODO: Do we care about the actual King that is castling?
    pub fn set_castling_queen(&mut self) {
        self.castling = Some(Castling::Queen);
        self.set_piece(Piece::white_king(E1).unwrap());
        self.set_move_type(MoveType::Castle);
    }

    /// Set pawn move.
    ///
    /// # Panics
    ///
    /// Panics if a `Piece` could not be constructed. This function should never panic in
    /// this case.
    pub fn set_pawn_move(&mut self) {
        self.set_piece(Piece::try_from("P").unwrap());
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
    ///
    /// # Errors
    ///
    /// * Returns an error if the move type is invalid for capture.
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
        self.check = Some(Check::Check);
    }

    /// Set check mate.
    pub fn set_check_mate(&mut self) {
        self.check = Some(Check::Mate);
    }

    /// Set promotion.
    pub fn set_promotion(&mut self, piece: Piece) {
        self.promotion = Some(piece);
    }

    /// Set `from_coord`
    pub fn set_from_coord(&mut self, coord: Option<Coord>) {
        self.from_coord = coord;
    }

    /// Set the `from_coord_file` index (0..=7).
    ///
    /// # Errors
    ///
    /// Returns a `ChuiError` result if the file index is out of range. See `set_file()`.
    pub fn set_from_coord_file(&mut self, file: char) -> ChuiResult<()> {
        if let Some(idx) = self.match_file_to_index(file) {
            self.from_coord_file = NonMaxU8::try_from(idx).ok();
            Ok(())
        } else {
            Err(ChuiError::InvalidFile(format!(
                "File `{}` is not valid",
                file
            )))
        }
    }

    /// Set the `from_coord_rank` index (0..=7).
    ///
    /// # Errors
    ///
    /// Returns a `ChuiError` result if the rank index is out of range. See `set_rank()`.
    pub fn set_from_coord_rank(&mut self, rank: char) -> ChuiResult<()> {
        if let Some(idx) = self.match_rank_to_index(rank) {
            self.from_coord_rank = NonMaxU8::try_from(idx).ok();
            Ok(())
        } else {
            Err(ChuiError::InvalidRank(format!(
                "Rank `{}` is not valid",
                rank
            )))
        }
    }

    /// Set `to_coord`
    pub fn set_to_coord(&mut self, coord: Option<Coord>) {
        self.to_coord = coord;
    }

    /// Set the `to_coord_file` index (0..=7).
    ///
    /// # Errors
    ///
    /// Returns a `ChuiError` result if the file index is out of range. See `set_file()`.
    pub fn set_to_coord_file(&mut self, file: char) -> ChuiResult<()> {
        if let Some(idx) = self.match_file_to_index(file) {
            self.to_coord_file = NonMaxU8::try_from(idx).ok();
            Ok(())
        } else {
            Err(ChuiError::InvalidFile(format!(
                "File `{}` is not valid",
                file
            )))
        }
    }

    /// Set the `to_coord_rank` index (0..=7).
    ///
    /// # Errors
    ///
    /// Returns a `ChuiError` result if the rank index is out of range. See `set_rank()`.
    pub fn set_to_coord_rank(&mut self, rank: char) -> ChuiResult<()> {
        if let Some(idx) = self.match_rank_to_index(rank) {
            self.to_coord_rank = NonMaxU8::try_from(idx).ok();
            Ok(())
        } else {
            Err(ChuiError::InvalidRank(format!(
                "Rank `{}` is not valid",
                rank
            )))
        }
    }

    /// Set possible en passant information.
    ///
    /// # Errors
    ///
    /// When:
    ///  * The move is not a Pawn move
    ///  * The Pawn move does not begin and end on the correct file or rank
    pub fn set_en_passant(&mut self, board: &Board) -> ChuiResult<()> {
        // 2nd and 7th ranks
        let from_rank: u8 = if self.to_move == Color::White { 1 } else { 6 };
        // 4th and 5th ranks
        let to_rank: u8 = if self.to_move == Color::White { 3 } else { 4 };
        // Calculate rank direction of en passant.
        let direction: i8 = if self.to_move == Color::White { -1 } else { 1 };
        // Calculate rank string ordinals.
        let from_rank_ord: &str = if from_rank + 1 == 2 { "nd" } else { "th" };
        let to_rank_ord: &str = "th";

        // Make sure this is a Pawn move.
        if !self.is_pawn_move() {
            return Err(ChuiError::InvalidEnPassant(
                "The move must be a Pawn move in order to determine en passant".to_string(),
            ));
        }

        let (from_coord, to_coord) =
            if let (Some(from_coord), Some(to_coord)) = (self.from_coord, self.to_coord) {
                (from_coord, to_coord)
            } else {
                return Err(ChuiError::InvalidEnPassant(
                    "Both the Move's `from_coord` and `to_coord` attributes must be set to \
                some coordinate."
                        .to_string(),
                ));
            };

        // Make sure this Pawn move starts and ends on the correct file.
        if from_coord.get_file() != to_coord.get_file() {
            return Err(ChuiError::InvalidEnPassant(
                "The Pawn move must begin and end on the same file".to_string(),
            ));
        }

        // Make sure this Pawn move starts and ends on the correct ranks.
        if !(from_coord.get_rank() == from_rank && to_coord.get_rank() == to_rank) {
            return Err(ChuiError::InvalidEnPassant(format!(
                "The Pawn move must originate from the {}{} rank and end on the {}{} rank for {}",
                from_rank + 1,
                from_rank_ord,
                to_rank + 1,
                to_rank_ord,
                self.to_move,
            )));
        }

        // Get the two possible Pawns that would enable en passant.
        let (ep_pawn1, ep_pawn2) = (
            board
                .get_position()
                .get_piece(Coord::try_from((to_coord.get_file() - 1, to_coord.get_rank())).ok()),
            board
                .get_position()
                .get_piece(Coord::try_from((to_coord.get_file() + 1, to_coord.get_rank())).ok()),
        );

        // If either piece is a pawn, then this is a valid en passant situation.
        if ep_pawn1.is_some_and(|p| p.is_pawn() && p.is_opposite_color(self.to_move))
            || ep_pawn2.is_some_and(|p| p.is_pawn() && p.is_opposite_color(self.to_move))
        {
            self.en_passant = Coord::try_from((
                to_coord.get_file(),
                (to_coord.get_rank() as i8 + direction) as u8,
            ))
            .ok();
        } else {
            return Err(ChuiError::InvalidEnPassant(
                "There are no pawns that can move en passant next turn".to_string(),
            ));
        }

        // Check to see if en passant square was set correctly.
        if self.en_passant.is_none() {
            return Err(ChuiError::Unknown(
                "En Passant target square was supposed to be set, but it wasn't".to_string(),
            ));
        }

        Ok(())
    }

    //
    // Updaters.
    //

    /// Mogrify the internal state of the Chess Move for application from the [`Board`] and the
    /// [`Game`].
    ///
    /// # Errors
    ///
    /// Returns a [`ChuiError`] result when `self.move_type` is None.
    pub fn process_move(&mut self, game: &mut Game) -> ChuiResult<()> {
        self.set_color(game.to_move);
        match self.move_type {
            Some(MoveType::Castle) => {
                println!("Move Type is Castle.");
            }
            Some(MoveType::PawnCapture) => {
                println!("Move Type is Pawn Capture.");
            }
            Some(MoveType::PawnMove) => {
                println!("Move Type is Pawn Move");
            }
            Some(MoveType::PieceCapture) => {
                println!("Move Type is Piece Capture.");
            }
            Some(MoveType::PieceMove) => {
                println!("Move Type is Piece Move.");
            }
            None => {
                println!("Move Type is Invalid.");
            }
        }

        Ok(())
    }

    /// Validate the chess move for the given `Position`.
    ///
    /// Depending on the parser, certain coordinates and attributes will
    /// bet set. Do not assume that this chess move has been previously modified by a `Parser`.
    /// For example, with an `ICCFParser`, the `from_coord` and `to_coord` will be
    /// set, but for an `AlgebraicParser` the move "e4" won't have `from_coord` set,
    /// only the `to_coord` attribute will be set properly, as well as the
    /// `move_type` attribute.
    ///
    /// # Errors
    ///
    /// When an invalid move is found.
    ///
    /// # Panics
    ///
    /// Should not panic.
    pub fn validate_move_for_board(&mut self, board: &Board) -> ChuiResult<()> {
        let _position = board.get_position();

        // 1) let's validate that this object instance has the necessary information to
        // determine that a move can at all be unambiguously played given the `Board`'s `Position`.

        // 1.a) All parsers will have set the `to_coord` `Coordinate`. In either case,
        // if `to_coord` is not set, there's no way to determine a valid move.
        if self.to_coord.is_none() {
            return Err(ChuiError::InvalidMove(
                "This Move's `to_coord` attribute is <None>; there's no way to determine \
                a valid move"
                    .to_string(),
            ));
        }

        // 1.b) Since `to_coord` is set, either the `from_coord` has to be set or the following
        // must be set:
        //  * `from_piece` must be set
        //  * `move_type` must be set
        if !(self.from_coord.is_some() || self.from_piece.is_some() && self.move_type.is_some()) {
            return Err(ChuiError::InvalidMove(
                "This Move's `from_coord` attribute is not set or `from_piece` is not set and the \
                `move_type` is not valid"
                    .to_string(),
            ));
        }

        // 2) If the `from_coord` and `to_coord` are set, we can process the board position and
        // normalize any values improperly set by any parser or via manual entry. If instead the
        // `from_piece` is set and the `move_type` is valid, we can determine if there is an
        // unambiguous `from_coord` for which to move the piece from.
        //
        // TODO: `from_piece` has its coordinate set. Program logic around not caring what a
        //       `Piece`'s coordinates are.

        // 2.a) Determine if `from_piece` is same color as `to_move`.
        if let Some(from_piece) = self.from_piece {
            if self.from_coord.is_none() && from_piece != self.to_move {
                return Err(ChuiError::InvalidMove(
                    "The piece that would be moved is not the same color as the player \
                    to move"
                        .to_string(),
                ));
            }
        }

        // 2.b) Get a vector of coordinates that `from_piece` is attacking. Hopefully one of
        // those coordinates are the same as `to_coord`.
        if let Some(from_piece) = self.from_piece {
            if from_piece
                .get_move_coords(board, None)
                .iter()
                .filter(|&&c| Some(c) == self.to_coord)
                .collect::<Vec<&Coord>>()
                .is_empty()
            {
                return Err(ChuiError::InvalidMove(format!(
                    "The piece `{}` cannot move to or capture the square `{}`",
                    from_piece,
                    self.to_coord.unwrap()
                )));
            }
        }

        // // Validate that the move is even possible.
        // if let Err(e) = self.from_coord.validate_possible_move(self.to_coord) {
        //     self.validation_errors.push(e.clone());
        //     return Err(e); // This error will block validation, return this error.
        // }

        // // This is usually true if the input parser is an `ICCFParser`.
        // if self.from_coord != Coord::zero() && self.to_coord != Coord::zero() {
        //     if let Some(piece_to_move) = position.get_piece(Some(self.from_coord)) {
        //         self.set_piece(piece_to_move);

        //         if let Some(opponent_piece) = position.get_piece(Some(self.to_coord)) {
        //             let move_coords = piece_to_move.get_move_coords(board, None);
        //             if move_coords.contains(&self.to_coord) {
        //                 self.set_capture()?;
        //             }

        //             return Err(ChuiError::InvalidCapture(format!(
        //                 "Cannot capture piece from {} to {}", self.from_coord, self.to_coord
        //             )))
        //         }

        //         self.set_en_passant(board);
        //         self.set_check();

        //         return Ok(());
        //     };
        // }

        // if self.from_coord == self.to_coord {
        //     return self
        //         .invalidate("The move's from coordinate cannot be the same as its to coordinate.");
        // }

        self.invalidate("Could not validate move")
    }

    //
    // Errors.
    //

    /// Invalidate
    ///
    /// # Errors
    ///
    /// Always errors. Could not validate the chess move.
    pub fn invalidate(&mut self, reason: &str) -> ChuiResult<()> {
        *self = ChessMove::new(self.to_move);
        Err(ChuiError::InvalidMove(reason.to_string()))
    }
}
