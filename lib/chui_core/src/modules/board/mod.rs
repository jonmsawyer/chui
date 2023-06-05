//! Board module.

// use std::ops::Index;

// use nonmax::NonMaxU8;

#[allow(unused_imports)]
use crate::constants::*;
use crate::{ChuiError, ChuiResult, Color, Coord, Move, Piece, PieceKind, FILES, RANKS};

/// The various chess variants available in Chui.
#[derive(Debug, Copy, Clone)]
pub enum ChessVariant {
    /// Standard Chess is the default chess variant. Used in all tournaments
    /// and official gameplay.
    StandardChess,
    //Chess960,
}

/// This struct represents the chessboard. Has a field called `board` which
/// references an 8x8 board. Has a field called `en_passant` which represents the en passant
/// target square.
#[derive(Debug, Clone, Copy)]
pub struct Board {
    /// Represents an 8x8 chessboard using nested arrays.
    board: [[Option<Piece>; FILES as usize]; RANKS as usize],

    /// Represents the en passant target square coordinate.
    en_passant_target_square: Option<Coord>,

    /// Represents the en passant target piece (pawn).
    en_passant_target_piece: Option<Piece>,
}

impl Board {
    //
    // Constructors.
    //

    /// Return a new `Board` given a chess variant.
    pub fn new(variant: ChessVariant) -> Board {
        match variant {
            ChessVariant::StandardChess => Board {
                board: Board::new_standard_chess(),
                en_passant_target_square: None,
                en_passant_target_piece: None,
            },
        }
    }

    /// New standard chess setup.
    pub fn new_standard_chess() -> [[Option<Piece>; FILES as usize]; RANKS as usize] {
        [
            // rank 1
            Board::standard_row_of_pieces(Color::White, 0).unwrap(),
            // rank 2
            [
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::White,
                    Coord::new(0, 1).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::White,
                    Coord::new(1, 1).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::White,
                    Coord::new(2, 1).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::White,
                    Coord::new(3, 1).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::White,
                    Coord::new(4, 1).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::White,
                    Coord::new(5, 1).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::White,
                    Coord::new(6, 1).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::White,
                    Coord::new(7, 1).unwrap(),
                )),
            ],
            [None; FILES as usize], // rank 3
            [None; FILES as usize], // rank 4
            [None; FILES as usize], // rank 5
            [None; FILES as usize], // rank 6
            // rank 7
            [
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::Black,
                    Coord::new(0, 6).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::Black,
                    Coord::new(1, 6).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::Black,
                    Coord::new(2, 6).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::Black,
                    Coord::new(3, 6).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::Black,
                    Coord::new(4, 6).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::Black,
                    Coord::new(5, 6).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::Black,
                    Coord::new(6, 6).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::Black,
                    Coord::new(7, 6).unwrap(),
                )),
            ],
            // rank 8
            Board::standard_row_of_pieces(Color::Black, 7).unwrap(),
        ]
    }

    //
    // Conditionals.
    //

    /// Apply the passed in move onto the chessboard.
    ///
    /// # Panics
    ///
    /// * Panics if the current move is `None` after returning a `ChuiError`.
    /// * Panics if the piece to move is `None` after returning a `ChuiError`.
    ///
    /// # Errors
    ///
    /// * Errors if the piece we're moving is `None`.
    pub fn apply_move(&mut self, current_move: &Option<Move>) -> ChuiResult<()> {
        if current_move.is_none() {
            return Err(ChuiError::InvalidMove(
                "No piece to apply move.".to_string(),
            ));
        }

        let move_obj = current_move.as_ref().expect("Current move cannot be None.");

        if move_obj.get_piece().is_none() {
            return Err(ChuiError::InvalidMove(
                "No piece to apply move.".to_string(),
            ));
        }

        let pieces = self.get_pieces(
            &mut move_obj
                .get_piece()
                .expect("The piece to move cannot be None."),
        );

        // println!("Pieces: {:?}", pieces);

        let mut pieces_can_move = Vec::<Piece>::new();

        for piece in pieces.iter() {
            if piece.get_move_coords(self).iter().any(|&coord| {
                coord.get_file() == move_obj.to_coord.unwrap().get_file()
                    && coord.get_rank() == move_obj.to_coord.unwrap().get_rank()
            }) {
                pieces_can_move.push(*piece);
            }
        }

        // println!("Pieces can move: {:?}", pieces_can_move);

        let (file, rank) = move_obj.to_coord.unwrap().to_u8_index();

        if pieces_can_move.is_empty() {
            Err(ChuiError::InvalidMove(format!(
                "No {} can move to target square {}{}",
                move_obj
                    .get_piece()
                    .expect("The piece to move cannot be None."),
                file,
                rank
            )))
        } else if pieces_can_move.len() == 1 {
            self.replace_piece(&mut pieces_can_move[0], move_obj);
            Ok(())
        } else {
            Err(ChuiError::InvalidMove(format!(
                "Ambiguous move. More than one piece can \
                        move to target square {}{}",
                file, rank
            )))
        }
    }

    /// Replace the given piece from one square to another.
    pub fn replace_piece(&mut self, piece_from: &mut Piece, move_obj: &Move) {
        let from_coord = piece_from.get_coord();
        let to_coord = move_obj.to_coord.unwrap();

        piece_from.set_coord(to_coord);

        self.board[from_coord.get_rank() as usize][from_coord.get_file() as usize] = None;
        self.board[to_coord.get_rank() as usize][to_coord.get_file() as usize] = Some(*piece_from);
    }

    //
    // Getters.
    //

    /// Get a refereance to the board.
    pub const fn get_board(&self) -> &[[Option<Piece>; FILES as usize]; RANKS as usize] {
        &self.board
    }

    /// Get the piece in the defined indicies. Remember that
    /// this is index-based, not Coordinate-based.
    pub fn get_piece(&self, coord: Coord) -> Option<Piece> {
        self.board[coord.get_rank() as usize][coord.get_file() as usize]
    }

    /// Get the available `Piece`s for a `Color`.
    ///
    /// # Panics
    ///
    /// * Panics when `some_piece` is None after checking that it is Some.
    pub fn get_pieces(&self, piece: &mut Piece) -> Vec<Piece> {
        let mut pieces = Vec::<Piece>::new();

        for (_, rank_arr) in self.board.iter().enumerate() {
            for (_, some_piece) in rank_arr.iter().enumerate() {
                if some_piece.is_some() {
                    let some_piece = some_piece.expect("Piece cannot be None.");

                    if some_piece.get_kind() == piece.get_kind()
                        && some_piece.get_color() == piece.get_color()
                    {
                        pieces.push(some_piece);
                    }
                }
            }
        }

        pieces
    }

    /// Get the en passant target square coordinate.
    pub fn get_en_passant_coord(&self) -> Option<Coord> {
        self.en_passant_target_square
    }

    /// Get the en passant target piece.
    pub fn get_en_passant_piece(&self) -> Option<Piece> {
        self.en_passant_target_piece
    }

    //
    // Setters.
    //

    /// Set the en passant target square coordinate.
    pub fn set_en_passant_coord(&mut self, coord: Coord) {
        self.en_passant_target_square = Some(coord);
    }

    /// Unset the en passant target square coordinate.
    pub fn unset_en_passant_coord(&mut self) {
        self.en_passant_target_square = None;
    }

    /// Set the en passant target square coordinate.
    pub fn set_en_passant_piece(&mut self, piece: Piece) {
        self.en_passant_target_piece = Some(piece);
    }

    /// Unset the en passant target square coordinate.
    pub fn unset_en_passant_piece(&mut self) {
        self.en_passant_target_piece = None;
    }

    /// Set both the en passant target square and piece.
    pub fn set_en_passant(&mut self, coord: Coord, piece: Piece) {
        self.set_en_passant_coord(coord);
        self.set_en_passant_piece(piece);
    }

    /// Unset the en passant target square and piece.
    pub fn unset_en_passant(&mut self) {
        self.unset_en_passant_coord();
        self.unset_en_passant_piece();
    }

    /// Set the Coordinates for all `Piece`s.
    ///
    /// # Errors
    ///
    /// * Errors when an invalid [`Coord`] is created.
    pub fn set_coords(&mut self) -> ChuiResult<()> {
        for (rank_idx, rank_arr) in self.board.iter_mut().enumerate() {
            for (file_idx, piece) in rank_arr.iter_mut().enumerate() {
                if piece.is_some() {
                    let piece = piece.as_mut().unwrap();
                    piece.set_coord(Coord::new(file_idx as u8, rank_idx as u8)?);
                }
            }
        }

        Ok(())
    }

    //
    // Piece move Coords.
    //

    /// Get a King's available move Coordinates.
    pub fn get_king_move_coords(&self, piece: &Piece) -> Vec<Coord> {
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

    /// Get a Queen's available move Coordinates.
    pub fn get_queen_move_coords(&self, piece: &Piece) -> Vec<Coord> {
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
    pub fn get_rook_move_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        coords.extend(self.get_top_coords(piece));
        coords.extend(self.get_right_coords(piece));
        coords.extend(self.get_bottom_coords(piece));
        coords.extend(self.get_left_coords(piece));

        coords
    }

    /// Get a Bishop's available move Coordinates.
    pub fn get_bishop_move_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        coords.extend(self.get_top_left_coords(piece));
        coords.extend(self.get_top_right_coords(piece));
        coords.extend(self.get_bottom_right_coords(piece));
        coords.extend(self.get_bottom_left_coords(piece));

        coords
    }

    /// Get a Knight's available move Coordinates.
    pub fn get_knight_move_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let file_idx = piece.get_file();
        let rank_idx = piece.get_rank();

        if let Ok(n_coord) = Coord::new(file_idx + 1, rank_idx + 2) {
            if let Some(n_piece) = self.get_piece(n_coord) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord)
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx + 1, rank_idx.wrapping_sub(2)) {
            if let Some(n_piece) = self.get_piece(n_coord) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord)
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx.wrapping_sub(1), rank_idx + 2) {
            if let Some(n_piece) = self.get_piece(n_coord) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord)
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx.wrapping_sub(1), rank_idx.wrapping_sub(2)) {
            if let Some(n_piece) = self.get_piece(n_coord) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord)
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx + 2, rank_idx + 1) {
            if let Some(n_piece) = self.get_piece(n_coord) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord)
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx + 2, rank_idx.wrapping_sub(1)) {
            if let Some(n_piece) = self.get_piece(n_coord) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord)
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx.wrapping_sub(2), rank_idx + 1) {
            if let Some(n_piece) = self.get_piece(n_coord) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord)
                }
            } else {
                coords.push(n_coord);
            }
        }

        if let Ok(n_coord) = Coord::new(file_idx.wrapping_sub(2), rank_idx.wrapping_sub(1)) {
            if let Some(n_piece) = self.get_piece(n_coord) {
                if n_piece.get_color() != piece.get_color() {
                    coords.push(n_coord)
                }
            } else {
                coords.push(n_coord);
            }
        }

        coords
    }

    /// Get a Pawn's available move Coordinates. Also accounts for en passant.
    pub fn get_pawn_move_coords(&self, piece: &Piece) -> Vec<Coord> {
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

                if let Some(en_passant) = self.get_en_passant_coord() {
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

                if let Some(en_passant) = self.get_en_passant_coord() {
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

                if let Some(en_passant) = self.get_en_passant_coord() {
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

                if let Some(en_passant) = self.get_en_passant_coord() {
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
    pub fn get_top_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut max_counter: u8 = 0;
        let mut rank_idx = piece.get_rank() + 1; // we're moving North

        while max_counter < piece.get_move_max() {
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
            max_counter += 1;
        }
        println!("Top Coords: {:?}", coords);
        coords
    }

    /// Get any Coordates East of the indicated indices that a piece can move.
    pub fn get_right_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut max_counter: u8 = 0;
        let mut file_idx = piece.get_file() + 1; // we're moving East

        while max_counter < piece.get_move_max() {
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
            max_counter += 1;
        }
        println!("Right Coords: {:?}", coords);
        coords
    }

    /// Get any Coordates South of the indicated indices that a piece can move.
    pub fn get_bottom_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut max_counter: u8 = 0;
        let mut rank_idx = piece.get_rank().wrapping_sub(1); // we're moving South

        while max_counter < piece.get_move_max() {
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
            max_counter += 1;
        }
        println!("Bottom Coords: {:?}", coords);
        coords
    }

    /// Get any Coordates West of the indicated indices that a piece can move.
    pub fn get_left_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut max_counter: u8 = 0;
        let mut file_idx = piece.get_file().wrapping_sub(1); // we're moving West

        while max_counter < piece.get_move_max() {
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
            max_counter += 1;
        }
        println!("Left Coords: {:?}", coords);
        coords
    }

    /// Get any Coordates North West of the indicated indices that a piece can move.
    pub fn get_top_left_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut max_counter: u8 = 0;
        let mut file_idx = piece.get_file().wrapping_sub(1); // we're moving West
        let mut rank_idx = piece.get_rank() + 1; // we're moving North

        while max_counter < piece.get_move_max() {
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
            max_counter += 1;
        }
        println!("Top Left Coords: {:?}", coords);
        coords
    }

    /// Get any Coordates North East of the indicated indices that a piece can move.
    pub fn get_top_right_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut max_counter: u8 = 0;
        let mut file_idx = piece.get_file() + 1; // we're moving East
        let mut rank_idx = piece.get_rank() + 1; // we're moving North

        while max_counter < piece.get_move_max() {
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
            max_counter += 1;
        }
        println!("Top Right Coords: {:?}", coords);
        coords
    }

    /// Get any Coordates South East of the indicated indices that a piece can move.
    pub fn get_bottom_right_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut max_counter: u8 = 0;
        let mut file_idx = piece.get_file() + 1; // we're moving East
        let mut rank_idx = piece.get_rank().wrapping_sub(1); // we're moving South

        while max_counter < piece.get_move_max() {
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
            max_counter += 1;
        }
        println!("Bottom Right Coords: {:?}", coords);
        coords
    }

    /// Get any Coordates South West of the indicated indices that a piece can move.
    pub fn get_bottom_left_coords(&self, piece: &Piece) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut max_counter: u8 = 0;
        let mut file_idx = piece.get_file().wrapping_sub(1); // we're moving West
        let mut rank_idx = piece.get_rank().wrapping_sub(1); // we're moving South

        while max_counter < piece.get_move_max() {
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
            max_counter += 1;
        }
        println!("Bottom Left Coords: {:?}", coords);
        coords
    }

    //
    // Utilities.
    //

    /// Produces a row (`[Option<Piece>; FILES]`) of pieces
    /// according their color.
    pub fn standard_row_of_pieces(
        color: Color,
        rank_idx: u8,
    ) -> ChuiResult<[Option<Piece>; FILES as usize]> {
        if rank_idx <= 7 {
            Ok([
                Some(Piece::new(PieceKind::Rook, color, Coord::new(0, rank_idx)?)),
                Some(Piece::new(
                    PieceKind::Knight,
                    color,
                    Coord::new(1, rank_idx)?,
                )),
                Some(Piece::new(
                    PieceKind::Bishop,
                    color,
                    Coord::new(2, rank_idx)?,
                )),
                Some(Piece::new(
                    PieceKind::Queen,
                    color,
                    Coord::new(3, rank_idx)?,
                )),
                Some(Piece::new(PieceKind::King, color, Coord::new(4, rank_idx)?)),
                Some(Piece::new(
                    PieceKind::Bishop,
                    color,
                    Coord::new(5, rank_idx)?,
                )),
                Some(Piece::new(
                    PieceKind::Knight,
                    color,
                    Coord::new(6, rank_idx)?,
                )),
                Some(Piece::new(PieceKind::Rook, color, Coord::new(7, rank_idx)?)),
            ])
        } else {
            Err(ChuiError::InvalidRank(format!(
                "Rank index ({}) is out of range",
                rank_idx
            )))
        }
    }

    /// Test function to display the board colors by a straight
    /// index from `0..64` range.
    ///
    /// Thanks to Kromey <https://github.com/Kromey>.
    pub fn display_board_colors_by_index() {
        for idx in 0..64 {
            let color_id = ((idx / 8) % 2 + idx % 2) % 2;
            print!("{}  ", color_id);

            if (idx + 1) % 8 == 0 {
                println!();
            }
        }
    }

    /// Print move coordinates in a pretty way.
    pub fn print_coords(coords: &Vec<Coord>) {
        let mut c_string = String::new();
        for c in coords.iter() {
            if c_string.len() == 0 {
                c_string.push_str(format!("{}", c).as_str())
            } else {
                c_string.push_str(format!(", {}", c).as_str())
            }
        }
        println!(" > {} Move Coords: {}", coords.len(), c_string);
    }

    /// Print [`Piece`].
    pub fn print_piece(piece: &Piece) {
        println!(
            "Piece: {}, Coord: {} ({:?})",
            piece,
            piece.get_coord(),
            piece.get_coord()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_board() -> Board {
        Board::new(ChessVariant::StandardChess)
    }

    fn print_info(piece: &Piece, coords: &Vec<Coord>) {
        Board::print_piece(piece);
        Board::print_coords(coords);
    }

    fn get_piece(board: &Board, coord: Coord) -> ChuiResult<Piece> {
        board
            .get_piece(coord)
            .ok_or(ChuiError::InvalidPiece(format!(
                "Invalid piece on {}",
                coord
            )))
    }

    fn get_vars(coord: (char, u8)) -> ChuiResult<(Board, Piece)> {
        let (board, coord) = (new_board(), Coord::try_from(coord)?);
        Ok((board, get_piece(&board, coord)?))
    }

    fn assert_coords(expected_coords: &Vec<(char, u8)>, coords: &Vec<Coord>) -> ChuiResult<()> {
        assert_eq!(expected_coords.len(), coords.len());
        assert!(expected_coords.iter().all(|e_coord| {
            let e_coord = Coord::try_from(*e_coord).unwrap();
            coords.iter().any(|c| e_coord == *c)
        }));
        Ok(())
    }

    #[test]
    fn test_white_rook_on_a1() -> ChuiResult<()> {
        let (board, piece) = get_vars(A1)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_knight_on_b1() -> ChuiResult<()> {
        let (board, piece) = get_vars(B1)?;
        let expected_coords = vec![A3, C3];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_bishop_on_c1() -> ChuiResult<()> {
        let (board, piece) = get_vars(C1)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_queen_on_d1() -> ChuiResult<()> {
        let (board, piece) = get_vars(D1)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_king_on_e1() -> ChuiResult<()> {
        let (board, piece) = get_vars(E1)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_bishop_on_f1() -> ChuiResult<()> {
        let (board, piece) = get_vars(F1)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_knight_on_g1() -> ChuiResult<()> {
        let (board, piece) = get_vars(G1)?;
        let expected_coords = vec![F3, H3];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_rook_on_h1() -> ChuiResult<()> {
        let (board, piece) = get_vars(H1)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_a2() -> ChuiResult<()> {
        let (board, piece) = get_vars(A2)?;
        let expected_coords = vec![A3, A4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_b2() -> ChuiResult<()> {
        let (board, piece) = get_vars(B2)?;
        let expected_coords = vec![B3, B4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_c2() -> ChuiResult<()> {
        let (board, piece) = get_vars(C2)?;
        let expected_coords = vec![C3, C4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_d2() -> ChuiResult<()> {
        let (board, piece) = get_vars(D2)?;
        let expected_coords = vec![D3, D4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_e2() -> ChuiResult<()> {
        let (board, piece) = get_vars(E2)?;
        let expected_coords = vec![E3, E4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_f2() -> ChuiResult<()> {
        let (board, piece) = get_vars(F2)?;
        let expected_coords = vec![F3, F4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_g2() -> ChuiResult<()> {
        let (board, piece) = get_vars(G2)?;
        let expected_coords = vec![G3, G4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_white_pawn_on_h2() -> ChuiResult<()> {
        let (board, piece) = get_vars(H2)?;
        let expected_coords = vec![H3, H4];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_rook_on_a8() -> ChuiResult<()> {
        let (board, piece) = get_vars(A8)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_knight_on_b8() -> ChuiResult<()> {
        let (board, piece) = get_vars(B8)?;
        let expected_coords = vec![A6, C6];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_bishop_on_c8() -> ChuiResult<()> {
        let (board, piece) = get_vars(C8)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_queen_on_d8() -> ChuiResult<()> {
        let (board, piece) = get_vars(D8)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_king_on_e8() -> ChuiResult<()> {
        let (board, piece) = get_vars(E8)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_bishop_on_f8() -> ChuiResult<()> {
        let (board, piece) = get_vars(F8)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_knight_on_g8() -> ChuiResult<()> {
        let (board, piece) = get_vars(G8)?;
        let expected_coords = vec![F6, H6];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_rook_on_h8() -> ChuiResult<()> {
        let (board, piece) = get_vars(H8)?;
        let expected_coords = Vec::<(char, u8)>::new();
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_a7() -> ChuiResult<()> {
        let (board, piece) = get_vars(A7)?;
        let expected_coords = vec![A6, A5];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_b7() -> ChuiResult<()> {
        let (board, piece) = get_vars(B7)?;
        let expected_coords = vec![B6, B5];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_c7() -> ChuiResult<()> {
        let (board, piece) = get_vars(C7)?;
        let expected_coords = vec![C6, C5];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_d7() -> ChuiResult<()> {
        let (board, piece) = get_vars(D7)?;
        let expected_coords = vec![D6, D5];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_e7() -> ChuiResult<()> {
        let (board, piece) = get_vars(E7)?;
        let expected_coords = vec![E6, E5];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_f7() -> ChuiResult<()> {
        let (board, piece) = get_vars(F7)?;
        let expected_coords = vec![F6, F5];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_g7() -> ChuiResult<()> {
        let (board, piece) = get_vars(G7)?;
        let expected_coords = vec![G6, G5];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }

    #[test]
    fn test_black_pawn_on_h7() -> ChuiResult<()> {
        let (board, piece) = get_vars(H7)?;
        let expected_coords = vec![H6, H5];
        let coords = piece.get_move_coords(&board);
        print_info(&piece, &coords);
        assert_coords(&expected_coords, &coords)?;
        Ok(())
    }
}
