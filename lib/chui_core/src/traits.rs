//! Chui Trait Definitions

use crate::prelude::{coord::*, *};
pub use chui_macros::Coordinate;

/// Coordinate trait.
trait Coordinate {}

/// Implement this trait to define the `parse()` method on a parser.
/// Any struct implementing this trait should parse a chess move
/// in an expected notation and return a `Move` object, representing
/// the validty or invalidity of the requested move for the given
/// chessboard.
///
/// Example:
///
/// ```
/// use chui_core::{
///     Move, Color, Piece, parser::Parser,
///     ChuiResult, ChuiError, Game, Coord
/// };
///
/// pub struct MyParser;
///
/// impl Parser for MyParser {
///     fn parse(&mut self, _the_move: String, _color: Color)
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
///
///     fn generate_move_from_board_coordinates(
///         &self,
///         engine: &Game,
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
    fn parse(&mut self, the_move: String, to_move: Color) -> ChuiResult<Move>;

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
        engine: &Game,
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
    fn trim_and_check_whitespace(&self, the_move: String) -> ChuiResult<String> {
        let the_move = the_move.trim().to_string();

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

/// trait Position.
pub trait Position {
    /// Get the piece at the given coordinate.
    fn get_piece(&self, coord: Coord) -> Option<Piece>;

    /// Get the available [`Piece`]s for a [`Color`].
    fn get_pieces(&self, piece: Piece) -> Vec<Piece>;

    /// Take a piece off of the board.
    fn take_piece(&mut self, coord: Coord) -> Option<Piece> {
        self.put_piece(None, coord)
    }

    /// Put a piece onto the board. Return any piece on the given square if it's occupied
    /// already.
    fn put_piece(&mut self, piece: Option<Piece>, coord: Coord) -> Option<Piece>;

    /// Replace the given piece from one square to another.
    ///
    /// # Errors
    ///
    /// This method does not error.
    fn replace_piece(
        &mut self,
        mut piece_from: Piece,
        move_obj: &Move,
    ) -> ChuiResult<Option<Piece>> {
        self.take_piece(piece_from.get_coord());
        piece_from.set_coord(move_obj.to_coord);
        Ok(self.put_piece(Some(piece_from), move_obj.to_coord))
    }

    /// Get all [`Piece`]s attacking a given coordinate.
    fn get_pieces_attacking_coord(
        &self,
        _board: &Board,
        _piece: Piece,
        _coord: Coord,
    ) -> Vec<Piece> {
        Vec::<Piece>::new()
    }

    //
    // Piece move Coords.
    //

    #[allow(clippy::similar_names)]
    /// Get a King's available move Coordinates.
    ///
    /// # Panics
    ///
    /// This method should never panic because of the use of defined constants.
    fn get_king_move_coords(&self, board: &Board, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        coords.extend(self.get_top_left_coords(piece));
        coords.extend(self.get_top_coords(piece));
        coords.extend(self.get_top_right_coords(piece));
        coords.extend(self.get_right_coords(piece));
        coords.extend(self.get_bottom_right_coords(piece));
        coords.extend(self.get_bottom_coords(piece));
        coords.extend(self.get_bottom_left_coords(piece));
        coords.extend(self.get_left_coords(piece));

        // Trim the move coordinates which are attacking the King.
        let mut coords: Vec<Coord> = coords
            .into_iter()
            .filter(|&coord| {
                self.get_pieces_attacking_coord(board, *piece, coord)
                    .is_empty()
            })
            .collect();

        // Add any valid castling coordinates
        match piece.get_color() {
            Color::White => {
                if board.white_can_castle_kingside {
                    let d1 = Coord::try_from(D1).unwrap();
                    let c1 = Coord::try_from(C1).unwrap();

                    if self.get_piece(d1).is_none() && self.get_piece(c1).is_none() {
                        let d1_pieces = self.get_pieces_attacking_coord(board, *piece, d1);
                        let c1_pieces = self.get_pieces_attacking_coord(board, *piece, c1);

                        if d1_pieces.is_empty() && c1_pieces.is_empty() {
                            coords.extend(vec![c1]);
                        }
                    }
                }

                if board.white_can_castle_queenside {
                    let f1 = Coord::try_from(F1).unwrap();
                    let g1 = Coord::try_from(G1).unwrap();

                    if self.get_piece(f1).is_none() && self.get_piece(g1).is_none() {
                        let f1_pieces = self.get_pieces_attacking_coord(board, *piece, f1);
                        let g1_pieces = self.get_pieces_attacking_coord(board, *piece, g1);

                        if f1_pieces.is_empty() && g1_pieces.is_empty() {
                            coords.extend(vec![g1]);
                        }
                    }
                }
            }
            Color::Black => {
                if board.black_can_castle_kingside {
                    let d8 = Coord::try_from(D8).unwrap();
                    let c8 = Coord::try_from(C8).unwrap();

                    if self.get_piece(d8).is_none() && self.get_piece(c8).is_none() {
                        let d8_pieces = self.get_pieces_attacking_coord(board, *piece, d8);
                        let c8_pieces = self.get_pieces_attacking_coord(board, *piece, c8);

                        if d8_pieces.is_empty() && c8_pieces.is_empty() {
                            coords.extend(vec![c8]);
                        }
                    }
                }

                if board.black_can_castle_queenside {
                    let f8 = Coord::try_from(F8).unwrap();
                    let g8 = Coord::try_from(G8).unwrap();

                    if self.get_piece(f8).is_none() && self.get_piece(g8).is_none() {
                        let f8_pieces = self.get_pieces_attacking_coord(board, *piece, f8);
                        let g8_pieces = self.get_pieces_attacking_coord(board, *piece, g8);

                        if f8_pieces.is_empty() && g8_pieces.is_empty() {
                            coords.extend(vec![g8]);
                        }
                    }
                }
            }
        }

        coords
    }

    /// Get a Queen's available move Coordinates.
    fn get_queen_move_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        coords.extend(self.get_top_left_coords(piece));
        coords.extend(self.get_top_coords(piece));
        coords.extend(self.get_top_right_coords(piece));
        coords.extend(self.get_right_coords(piece));
        coords.extend(self.get_bottom_right_coords(piece));
        coords.extend(self.get_bottom_coords(piece));
        coords.extend(self.get_bottom_left_coords(piece));
        coords.extend(self.get_left_coords(piece));

        coords
    }

    /// Get a Rook's available move Coordinates.
    fn get_rook_move_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        coords.extend(self.get_top_coords(piece));
        coords.extend(self.get_right_coords(piece));
        coords.extend(self.get_bottom_coords(piece));
        coords.extend(self.get_left_coords(piece));

        coords
    }

    /// Get a Bishop's available move Coordinates.
    fn get_bishop_move_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        coords.extend(self.get_top_left_coords(piece));
        coords.extend(self.get_top_right_coords(piece));
        coords.extend(self.get_bottom_right_coords(piece));
        coords.extend(self.get_bottom_left_coords(piece));

        coords
    }

    /// Get a Knight's available move Coordinates.
    fn get_knight_move_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let file_idx = piece.get_file();
        let rank_idx = piece.get_rank();

        if let Ok(n_coord) = Coord::new(file_idx + 1, rank_idx + 2) {
            if let Some(n_piece) = self.get_piece(n_coord) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord);
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx + 1, rank_idx.wrapping_sub(2)) {
            if let Some(n_piece) = self.get_piece(n_coord) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord);
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx.wrapping_sub(1), rank_idx + 2) {
            if let Some(n_piece) = self.get_piece(n_coord) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord);
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx.wrapping_sub(1), rank_idx.wrapping_sub(2)) {
            if let Some(n_piece) = self.get_piece(n_coord) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord);
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx + 2, rank_idx + 1) {
            if let Some(n_piece) = self.get_piece(n_coord) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord);
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx + 2, rank_idx.wrapping_sub(1)) {
            if let Some(n_piece) = self.get_piece(n_coord) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord);
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx.wrapping_sub(2), rank_idx + 1) {
            if let Some(n_piece) = self.get_piece(n_coord) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord);
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx.wrapping_sub(2), rank_idx.wrapping_sub(1)) {
            if let Some(n_piece) = self.get_piece(n_coord) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord);
                }
            } else {
                coords.push(n_coord);
            }
        }

        coords
    }

    #[allow(clippy::cognitive_complexity)]
    /// Get a Pawn's available move Coordinates. Also accounts for en passant.
    fn get_pawn_move_coords(&self, board: &Board, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let file_idx = piece.get_file();
        let rank_idx = piece.get_rank();

        if piece.get_color() == Color::White {
            let new_coord_1 = Coord::new(file_idx, rank_idx + 1);
            let new_coord_2 = Coord::new(file_idx, rank_idx + 2);

            if let Ok(new_coord) = new_coord_1 {
                if self.get_piece(new_coord).is_none() {
                    coords.push(new_coord);
                }
            }

            // Pawn starting rank for White.
            if let Ok(new_coord) = new_coord_1 {
                if let Ok(new_coord2) = new_coord_2 {
                    if rank_idx == 1
                        && self.get_piece(new_coord).is_none()
                        && self.get_piece(new_coord2).is_none()
                    {
                        coords.push(new_coord2);
                    }
                }
            }

            let capture_1 = Coord::new(file_idx.wrapping_sub(1), rank_idx + 1);
            let capture_2 = Coord::new(file_idx + 1, rank_idx + 1);

            if let Ok(capture_1) = capture_1 {
                if let Some(o_piece) = self.get_piece(capture_1) {
                    if o_piece.get_color() == Color::Black {
                        coords.push(capture_1);
                    }
                }

                if let Some(en_passant) = board.get_en_passant_coord() {
                    if capture_1 == en_passant {
                        coords.push(capture_1);
                    }
                }
            }

            if let Ok(capture_2) = capture_2 {
                if let Some(o_piece) = self.get_piece(capture_2) {
                    if o_piece.get_color() == Color::Black {
                        coords.push(capture_2);
                    }
                }

                if let Some(en_passant) = board.get_en_passant_coord() {
                    if capture_2 == en_passant {
                        coords.push(capture_2);
                    }
                }
            }
        } else {
            let new_coord = Coord::new(file_idx, rank_idx.wrapping_sub(1));
            let new_coord2 = Coord::new(file_idx, rank_idx.wrapping_sub(2));

            if let Ok(new_coord) = new_coord {
                if self.get_piece(new_coord).is_none() {
                    coords.push(new_coord);
                }
            }

            // Pawn starting rank for Black.
            if let Ok(new_coord) = new_coord {
                if let Ok(new_coord2) = new_coord2 {
                    if rank_idx == 6
                        && self.get_piece(new_coord).is_none()
                        && self.get_piece(new_coord2).is_none()
                    {
                        coords.push(new_coord2);
                    }
                }
            }

            let capture_1 = Coord::new(file_idx.wrapping_sub(1), rank_idx.wrapping_sub(1));
            let capture_2 = Coord::new(file_idx + 1, rank_idx.wrapping_sub(1));

            if let Ok(capture_1) = capture_1 {
                if let Some(o_piece) = self.get_piece(capture_1) {
                    if o_piece.get_color() == Color::White {
                        coords.push(capture_1);
                    }
                }

                if let Some(en_passant) = board.get_en_passant_coord() {
                    if capture_1 == en_passant {
                        coords.push(capture_1);
                    }
                }
            }

            if let Ok(capture_2) = capture_2 {
                if let Some(o_piece) = self.get_piece(capture_2) {
                    if o_piece.get_color() == Color::White {
                        coords.push(capture_2);
                    }
                }

                if let Some(en_passant) = board.get_en_passant_coord() {
                    if capture_2 == en_passant {
                        coords.push(capture_2);
                    }
                }
            }
        }

        coords
    }

    //
    // Position Coords.
    //

    /// Get any Coordates North of the indicated indices that a piece can move.
    fn get_top_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut rank_idx = piece.get_rank() + 1; // we're moving North

        for _ in 0..piece.get_move_max() {
            if let Ok(new_coord) = Coord::new(piece.get_file(), rank_idx) {
                if let Some(o_piece) = self.get_piece(new_coord) {
                    if !o_piece.is_same_color(*piece) {
                        coords.push(new_coord);
                    }

                    break;
                } else {
                    coords.push(new_coord);
                }
            } else {
                // Break here because further coordinates will be invalid.
                break;
            }

            rank_idx += 1;
        }

        coords
    }

    /// Get any Coordates East of the indicated indices that a piece can move.
    fn get_right_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut file_idx = piece.get_file() + 1; // we're moving East

        for _ in 0..piece.get_move_max() {
            if let Ok(new_coord) = Coord::new(file_idx, piece.get_rank()) {
                if let Some(o_piece) = self.get_piece(new_coord) {
                    if !o_piece.is_same_color(*piece) {
                        coords.push(new_coord);
                    }

                    break;
                } else {
                    coords.push(new_coord);
                }
            } else {
                // Break here because further coordinates will be invalid.
                break;
            }

            file_idx += 1;
        }

        coords
    }

    /// Get any Coordates South of the indicated indices that a piece can move.
    fn get_bottom_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut rank_idx = piece.get_rank().wrapping_sub(1); // we're moving South

        for _ in 0..piece.get_move_max() {
            if let Ok(new_coord) = Coord::new(piece.get_file(), rank_idx) {
                if let Some(o_piece) = self.get_piece(new_coord) {
                    if !o_piece.is_same_color(*piece) {
                        coords.push(new_coord);
                    }

                    break;
                } else {
                    coords.push(new_coord);
                }
            } else {
                // Break here because further coordinates will be invalid.
                break;
            }

            rank_idx = rank_idx.wrapping_sub(1);
        }

        coords
    }

    /// Get any Coordates West of the indicated indices that a piece can move.
    fn get_left_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut file_idx = piece.get_file().wrapping_sub(1); // we're moving West

        for _ in 0..piece.get_move_max() {
            if let Ok(new_coord) = Coord::new(file_idx, piece.get_rank()) {
                if let Some(o_piece) = self.get_piece(new_coord) {
                    if !o_piece.is_same_color(*piece) {
                        coords.push(new_coord);
                    }

                    break;
                } else {
                    coords.push(new_coord);
                }
            } else {
                // Break here because further coordinates will be invalid.
                break;
            }

            file_idx = file_idx.wrapping_sub(1);
        }

        coords
    }

    /// Get any Coordates North West of the indicated indices that a piece can move.
    fn get_top_left_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut file_idx = piece.get_file().wrapping_sub(1); // we're moving West
        let mut rank_idx = piece.get_rank() + 1; // we're moving North

        for _ in 0..piece.get_move_max() {
            if let Ok(new_coord) = Coord::new(file_idx, rank_idx) {
                if let Some(o_piece) = self.get_piece(new_coord) {
                    if !o_piece.is_same_color(*piece) {
                        coords.push(new_coord);
                    }

                    break;
                } else {
                    coords.push(new_coord);
                }
            } else {
                // Break here because further coordinates will be invalid.
                break;
            }

            file_idx = file_idx.wrapping_sub(1);
            rank_idx += 1;
        }

        coords
    }

    /// Get any Coordates North East of the indicated indices that a piece can move.
    fn get_top_right_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut file_idx = piece.get_file() + 1; // we're moving East
        let mut rank_idx = piece.get_rank() + 1; // we're moving North

        for _ in 0..piece.get_move_max() {
            if let Ok(new_coord) = Coord::new(file_idx, rank_idx) {
                if let Some(o_piece) = self.get_piece(new_coord) {
                    if !o_piece.is_same_color(*piece) {
                        coords.push(new_coord);
                    }

                    break;
                } else {
                    coords.push(new_coord);
                }
            } else {
                // Break here because further coordinates will be invalid.
                break;
            }

            file_idx += 1;
            rank_idx += 1;
        }

        coords
    }

    /// Get any Coordates South East of the indicated indices that a piece can move.
    fn get_bottom_right_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut file_idx = piece.get_file() + 1; // we're moving East
        let mut rank_idx = piece.get_rank().wrapping_sub(1); // we're moving South

        for _ in 0..piece.get_move_max() {
            if let Ok(new_coord) = Coord::new(file_idx, rank_idx) {
                if let Some(o_piece) = self.get_piece(new_coord) {
                    if !o_piece.is_same_color(*piece) {
                        coords.push(new_coord);
                    }

                    break;
                } else {
                    coords.push(new_coord);
                }
            } else {
                // Break here because further coordinates will be invalid.
                break;
            }

            file_idx += 1;
            rank_idx = rank_idx.wrapping_sub(1);
        }

        coords
    }

    /// Get any Coordates South West of the indicated indices that a piece can move.
    fn get_bottom_left_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut file_idx = piece.get_file().wrapping_sub(1); // we're moving West
        let mut rank_idx = piece.get_rank().wrapping_sub(1); // we're moving South

        for _ in 0..piece.get_move_max() {
            if let Ok(new_coord) = Coord::new(file_idx, rank_idx) {
                if let Some(o_piece) = self.get_piece(new_coord) {
                    if !o_piece.is_same_color(*piece) {
                        coords.push(new_coord);
                    }

                    break;
                } else {
                    coords.push(new_coord);
                }
            } else {
                // Break here because further coordinates will be invalid.
                break;
            }

            file_idx = file_idx.wrapping_sub(1);
            rank_idx = rank_idx.wrapping_sub(1);
        }

        coords
    }
}
