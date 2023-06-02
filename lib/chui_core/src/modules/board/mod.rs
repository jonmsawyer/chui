//! Board module.

// use std::ops::Index;

// use nonmax::NonMaxU8;

use crate::{ChuiError, ChuiResult, Color, Coord, Move, Piece, PieceKind, FILES, RANKS};

pub mod result;
pub use result::{BoardError, BoardResult};

/// The various chess variants available in Chui.
#[derive(Debug, Copy, Clone)]
pub enum ChessVariant {
    /// Standard Chess is the default chess variant. Used in all tournaments
    /// and official gameplay.
    StandardChess,
    //Chess960,
}

/// This struct represents the chessboard. Has one field called `board` which
/// references an 8x8 board.
#[derive(Debug, Clone, Copy)]
pub struct Board {
    /// Represents an 8x8 chessboard using nested arrays.
    board: [[Option<Piece>; FILES as usize]; RANKS as usize],
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
            if piece
                .get_move_coords(self, current_move)
                .iter()
                .any(|&coord| {
                    coord.get_file() == move_obj.to_coord.unwrap().get_file()
                        && coord.get_rank() == move_obj.to_coord.unwrap().get_rank()
                })
            {
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

                    if some_piece.get_piece() == piece.get_piece()
                        && some_piece.get_color() == piece.get_color()
                    {
                        pieces.push(some_piece);
                    }
                }
            }
        }

        pieces
    }

    //
    // Setters.
    //

    /// Set the Coordinates for all `Piece`s.
    ///
    /// # Panics
    ///
    /// * Panics when `piece` is None after checking that it is Some.
    ///
    /// # Errors
    ///
    /// * This function does not Error.
    pub fn set_coords(&mut self) -> BoardResult<()> {
        for (rank_idx, rank_arr) in self.board.iter_mut().enumerate() {
            for (file_idx, piece) in rank_arr.iter_mut().enumerate() {
                if piece.is_some() {
                    let piece = piece.as_mut().expect("Piece cannot be None.");
                    piece.set_coord(Coord::new(file_idx as u8, rank_idx as u8).unwrap());
                }
            }
        }

        Ok(())
    }

    //
    // Piece move Coords.
    //

    /// Get a King's available move Coordinates.
    pub fn get_king_move_coords(&self, coord: Coord, current_move: &Option<Move>) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        coords.extend(self.get_top_left_coords(coord, &mut 1, current_move));
        coords.extend(self.get_top_coords(coord, 1, false));
        coords.extend(self.get_top_right_coords(coord, &mut 1, current_move));
        coords.extend(self.get_right_coords(coord, &mut 1, false, current_move));
        coords.extend(self.get_bottom_right_coords(coord, &mut 1, current_move));
        coords.extend(self.get_bottom_coords(coord, 1, false));
        coords.extend(self.get_bottom_left_coords(coord, &mut 1, current_move));
        coords.extend(self.get_left_coords(coord, &mut 1, false, current_move));

        coords
    }

    /// Get a Queen's available move Coordinates.
    pub fn get_queen_move_coords(&self, coord: Coord, current_move: &Option<Move>) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut max = if FILES <= RANKS { RANKS } else { FILES };

        coords.extend(self.get_top_left_coords(coord, &mut max, current_move));
        coords.extend(self.get_top_coords(coord, max, false));
        coords.extend(self.get_top_right_coords(coord, &mut max, current_move));
        coords.extend(self.get_right_coords(coord, &mut max, false, current_move));
        coords.extend(self.get_bottom_right_coords(coord, &mut max, current_move));
        coords.extend(self.get_bottom_coords(coord, max, false));
        coords.extend(self.get_bottom_left_coords(coord, &mut max, current_move));
        coords.extend(self.get_left_coords(coord, &mut max, false, current_move));

        coords
    }

    /// Get a Rook's available move Coordinates.
    pub fn get_rook_move_coords(&self, coord: Coord, current_move: &Option<Move>) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut max = if FILES <= RANKS { RANKS } else { FILES };

        coords.extend(self.get_top_coords(coord, max, false));
        coords.extend(self.get_right_coords(coord, &mut max, false, current_move));
        coords.extend(self.get_bottom_coords(coord, max, false));
        coords.extend(self.get_left_coords(coord, &mut max, false, current_move));

        coords
    }

    /// Get a Bishop's available move Coordinates.
    pub fn get_bishop_move_coords(&self, coord: Coord, current_move: &Option<Move>) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let mut max = if FILES <= RANKS { RANKS } else { FILES };

        coords.extend(self.get_top_left_coords(coord, &mut max, current_move));
        coords.extend(self.get_top_right_coords(coord, &mut max, current_move));
        coords.extend(self.get_bottom_right_coords(coord, &mut max, current_move));
        coords.extend(self.get_bottom_left_coords(coord, &mut max, current_move));

        coords
    }

    /// Get a Knight's available move Coordinates.
    pub fn get_knight_move_coords(&self, coord: Coord) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();
        let file_idx = coord.get_file() as isize;
        let rank_idx = coord.get_rank() as isize;

        if file_idx + 1 < FILES as isize
            && rank_idx + 2 < RANKS as isize
            && self
                .get_piece(Coord::new(file_idx as u8 + 1, rank_idx as u8 + 2).unwrap())
                .is_none()
        {
            coords.push(Coord::new(file_idx as u8 + 1, rank_idx as u8 + 2).unwrap());
        }

        if file_idx + 1 < FILES as isize
            && rank_idx - 2 >= 0
            && self
                .get_piece(Coord::new(file_idx as u8 + 1, rank_idx as u8 - 2).unwrap())
                .is_none()
        {
            coords.push(Coord::new(file_idx as u8 + 1, rank_idx as u8 - 2).unwrap());
        }

        if file_idx > 0
            && rank_idx + 2 < RANKS as isize
            && self
                .get_piece(Coord::new(file_idx as u8 - 1, rank_idx as u8 + 2).unwrap())
                .is_none()
        {
            coords.push(Coord::new(file_idx as u8 - 1, rank_idx as u8 + 2).unwrap());
        }

        if file_idx > 0
            && rank_idx - 2 >= 0
            && self
                .get_piece(Coord::new(file_idx as u8 - 1, rank_idx as u8 - 2).unwrap())
                .is_none()
        {
            coords.push(Coord::new(file_idx as u8 - 1, rank_idx as u8 - 2).unwrap());
        }

        if file_idx + 2 < FILES as isize
            && rank_idx + 1 < RANKS as isize
            && self
                .get_piece(Coord::new(file_idx as u8 + 2, rank_idx as u8 + 1).unwrap())
                .is_none()
        {
            coords.push(Coord::new(file_idx as u8 + 2, rank_idx as u8 + 1).unwrap());
        }

        if file_idx + 2 < FILES as isize
            && rank_idx > 0
            && self
                .get_piece(Coord::new(file_idx as u8 + 2, rank_idx as u8 - 1).unwrap())
                .is_none()
        {
            coords.push(Coord::new(file_idx as u8 + 2, rank_idx as u8 - 1).unwrap());
        }

        if file_idx - 2 >= 0
            && rank_idx + 1 < RANKS as isize
            && self
                .get_piece(Coord::new(file_idx as u8 - 2, rank_idx as u8 + 1).unwrap())
                .is_none()
        {
            coords.push(Coord::new(file_idx as u8 - 2, rank_idx as u8 + 1).unwrap());
        }

        if file_idx - 2 >= 0
            && rank_idx > 0
            && self
                .get_piece(Coord::new(file_idx as u8 - 2, rank_idx as u8 - 1).unwrap())
                .is_none()
        {
            coords.push(Coord::new(file_idx as u8 - 2, rank_idx as u8 - 1).unwrap());
        }

        coords
    }

    /// Get a Pawn's available move Coordinates.
    pub fn get_pawn_move_coords(&self, coord: Coord, color: Color) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        if let Color::White = color {
            let new_coord = Coord::new(coord.get_file(), coord.get_rank() + 1).unwrap();
            let new_coord2 = Coord::new(coord.get_file(), coord.get_rank() + 2).unwrap();
            if coord.get_rank() + 1 < RANKS && self.get_piece(new_coord).is_none() {
                coords.push(new_coord);
            }

            // Pawn starting rank for White.
            if coord.get_rank() == 1
                && self.get_piece(new_coord).is_none()
                && self.get_piece(new_coord2).is_none()
            {
                coords.push(new_coord2);
            }
        } else {
            let new_coord = Coord::new(coord.get_file(), coord.get_rank() - 1).unwrap();
            let new_coord2 = Coord::new(coord.get_file(), coord.get_rank() - 2).unwrap();
            if coord.get_rank() - 1 > 0 && self.get_piece(new_coord).is_none() {
                coords.push(new_coord);
            }

            // Pawn starting rank for Black.
            if coord.get_rank() == 6
                && self.get_piece(new_coord).is_none()
                && self.get_piece(new_coord2).is_none()
            {
                coords.push(new_coord2);
            }
        }

        coords
    }

    //
    // Position Coords.
    //

    /// Get any Coordates North of the indicated indices that a piece can move.
    ///
    /// # Panics
    ///
    /// * Panics if the piece we're getting Coordinates for is None after checking
    ///   to see if was None previously.
    pub fn get_top_coords(&self, coord: Coord, limit: u8, ignore_pieces: bool) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        let mut limit_counter: u8 = 0;
        let mut rank_idx_counter = coord.get_rank() + 1;

        while rank_idx_counter < RANKS && limit_counter < limit {
            let new_coord = Coord::new(coord.get_file(), coord.get_rank() + 1).unwrap();
            if ignore_pieces || self.get_piece(new_coord).is_none() {
                coords.push(new_coord);
            } else {
                let move_coords = Piece::get_file_rank_from_coords(coord);
                println!("(Top) Breaking on {}{}", move_coords.0, move_coords.1);
                break;
            }

            rank_idx_counter += 1;
            limit_counter += 1;
        }

        coords
    }

    /// Get any Coordates East of the indicated indices that a piece can move.
    ///
    /// # Panics
    ///
    /// * Panics if the piece we're getting Coordinates for is None after checking
    ///   to see if was None previously.
    pub fn get_right_coords(
        &self,
        coord: Coord,
        limit: &mut u8,
        ignore_pieces: bool,
        current_move: &Option<Move>,
    ) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        let mut limit_counter: u8 = 0;
        let mut file_idx_counter = coord.get_file() + 1;

        if let Some(move_obj) = current_move {
            if let PieceKind::King = move_obj.piece.unwrap().get_piece() {
                *limit = 2;
            }
        }

        while file_idx_counter < FILES && limit_counter < *limit {
            if ignore_pieces
                || self
                    .get_piece(Coord::new(file_idx_counter, coord.get_rank()).unwrap())
                    .is_none()
            {
                coords.push(Coord::new(file_idx_counter, coord.get_rank()).unwrap());
            } else {
                let move_coords = Piece::get_file_rank_from_coords(
                    Coord::new(file_idx_counter, coord.get_rank()).unwrap(),
                );
                println!("(Right) Breaking on {}{}", move_coords.0, move_coords.1);
                break;
            }

            file_idx_counter += 1;
            limit_counter += 1;
        }

        coords
    }

    /// Get any Coordates South of the indicated indices that a piece can move.
    ///
    /// # Panics
    ///
    /// * Panics if the piece we're getting Coordinates for is None after checking
    ///   to see if was None previously.
    pub fn get_bottom_coords(&self, coord: Coord, limit: u8, ignore_pieces: bool) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        let mut limit_counter: u8 = 0;
        let mut rank_idx_counter = coord.get_rank() as isize - 1;

        while rank_idx_counter >= 0 && limit_counter < limit {
            let new_coord = Coord::new(coord.get_file(), rank_idx_counter as u8).unwrap();
            if ignore_pieces || self.get_piece(new_coord).is_none() {
                coords.push(new_coord);
            } else {
                let move_coords = Piece::get_file_rank_from_coords(coord);
                println!("(Bottom) Breaking on {}{}", move_coords.0, move_coords.1);
                break;
            }

            rank_idx_counter -= 1;
            limit_counter += 1;
        }

        coords
    }

    /// Get any Coordates West of the indicated indices that a piece can move.
    ///
    /// # Panics
    ///
    /// * Panics if the piece we're getting Coordinates for is None after checking
    ///   to see if was None previously.
    pub fn get_left_coords(
        &self,
        coord: Coord,
        limit: &mut u8,
        ignore_pieces: bool,
        current_move: &Option<Move>,
    ) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        let mut limit_counter: u8 = 0;
        let mut file_idx_counter = coord.get_file() as isize - 1;

        if let Some(move_obj) = current_move {
            if let PieceKind::King = move_obj.get_piece().unwrap().get_piece() {
                *limit = 2;
            }
        }

        while file_idx_counter >= 0 && limit_counter < *limit {
            let new_coord = Coord::new(file_idx_counter as u8, coord.get_rank()).unwrap();
            if ignore_pieces || self.get_piece(new_coord).is_none() {
                coords.push(new_coord);
            } else {
                let move_coords = Piece::get_file_rank_from_coords(new_coord);
                println!("(Left) Breaking on {}{}", move_coords.0, move_coords.1);
                break;
            }

            file_idx_counter -= 1;
            limit_counter += 1;
        }

        coords
    }

    /// Get any Coordates North West of the indicated indices that a piece can move.
    pub fn get_top_left_coords(
        &self,
        coord: Coord,
        limit: &mut u8,
        current_move: &Option<Move>,
    ) -> Vec<Coord> {
        Board::zip_top_left_coords(
            self,
            self.get_top_coords(coord, *limit, true),
            self.get_left_coords(coord, limit, true, current_move),
        )
    }

    /// Get any Coordates North East of the indicated indices that a piece can move.
    pub fn get_top_right_coords(
        &self,
        coord: Coord,
        limit: &mut u8,
        current_move: &Option<Move>,
    ) -> Vec<Coord> {
        Board::zip_top_right_coords(
            self,
            self.get_top_coords(coord, *limit, true),
            self.get_right_coords(coord, limit, true, current_move),
        )
    }

    /// Get any Coordates South East of the indicated indices that a piece can move.
    pub fn get_bottom_right_coords(
        &self,
        coord: Coord,
        limit: &mut u8,
        current_move: &Option<Move>,
    ) -> Vec<Coord> {
        Board::zip_bottom_right_coords(
            self,
            self.get_bottom_coords(coord, *limit, true),
            self.get_right_coords(coord, limit, true, current_move),
        )
    }

    /// Get any Coordates South West of the indicated indices that a piece can move.
    pub fn get_bottom_left_coords(
        &self,
        coord: Coord,
        limit: &mut u8,
        current_move: &Option<Move>,
    ) -> Vec<Coord> {
        Board::zip_bottom_left_coords(
            self,
            self.get_bottom_coords(coord, *limit, true),
            self.get_left_coords(coord, limit, true, current_move),
        )
    }

    //
    // Utilities.
    //

    /// Produces a row (`[Option<Piece>; FILES]`) of pieces
    /// according their color.
    pub fn standard_row_of_pieces(
        color: Color,
        rank_idx: u8,
    ) -> BoardResult<[Option<Piece>; FILES as usize]> {
        if rank_idx <= 7 {
            Ok([
                Some(Piece::new(
                    PieceKind::Rook,
                    color,
                    Coord::new(0, rank_idx).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Knight,
                    color,
                    Coord::new(1, rank_idx).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Bishop,
                    color,
                    Coord::new(2, rank_idx).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Queen,
                    color,
                    Coord::new(3, rank_idx).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::King,
                    color,
                    Coord::new(4, rank_idx).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Bishop,
                    color,
                    Coord::new(5, rank_idx).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Knight,
                    color,
                    Coord::new(6, rank_idx).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Rook,
                    color,
                    Coord::new(7, rank_idx).unwrap(),
                )),
            ])
        } else {
            Err(BoardError::InvalidRank(format!(
                "Rank index ({}) is out of range",
                rank_idx
            )))
        }
    }

    /// "Zips" together top Coords and left Coords.
    pub fn zip_top_left_coords(
        &self,
        top_coords: Vec<Coord>,
        left_coords: Vec<Coord>,
    ) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        for (rank_coord, file_coord) in top_coords.iter().zip(left_coords) {
            let new_coord = Coord::new(file_coord.get_file(), rank_coord.get_rank()).unwrap();
            if self.get_piece(new_coord).is_some() {
                let move_coords = Piece::get_file_rank_from_coords(new_coord);
                println!("(Top Left) Breaking on {}{}", move_coords.0, move_coords.1);
                break;
            }

            coords.push(new_coord);
        }

        coords
    }

    /// "Zips" together top Coords and right Coords.
    pub fn zip_top_right_coords(
        &self,
        top_coords: Vec<Coord>,
        right_coords: Vec<Coord>,
    ) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        for (rank_coord, file_coord) in top_coords.iter().zip(right_coords) {
            let new_coord = Coord::new(file_coord.get_file(), rank_coord.get_rank()).unwrap();
            if self.get_piece(new_coord).is_some() {
                let move_coords = Piece::get_file_rank_from_coords(new_coord);
                println!("(Top Right) Breaking on {}{}", move_coords.0, move_coords.1);
                break;
            }

            coords.push(new_coord);
        }

        coords
    }

    /// "Zips" together bottom Coords and right Coords.
    pub fn zip_bottom_right_coords(
        &self,
        bottom_coords: Vec<Coord>,
        right_coords: Vec<Coord>,
    ) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        for (rank_coord, file_coord) in bottom_coords.iter().zip(right_coords) {
            let new_coord = Coord::new(file_coord.get_file(), rank_coord.get_rank()).unwrap();
            if self.get_piece(new_coord).is_some() {
                let move_coords = Piece::get_file_rank_from_coords(new_coord);
                println!(
                    "(Bottom Right) Breaking on {}{}",
                    move_coords.0, move_coords.1
                );
                break;
            }

            coords.push(new_coord);
        }

        coords
    }

    /// "Zips" together bottom Coords and left Coords.
    pub fn zip_bottom_left_coords(
        &self,
        bottom_coords: Vec<Coord>,
        left_coords: Vec<Coord>,
    ) -> Vec<Coord> {
        let mut coords = Vec::<Coord>::new();

        for (rank_coord, file_coord) in bottom_coords.iter().zip(left_coords) {
            let new_coord = Coord::new(file_coord.get_file(), rank_coord.get_rank()).unwrap();
            if self.get_piece(new_coord).is_some() {
                let move_coords = Piece::get_file_rank_from_coords(new_coord);
                println!(
                    "(Bottom Left) Breaking on {}{}",
                    move_coords.0, move_coords.1
                );
                break;
            }

            coords.push(new_coord);
        }

        coords
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
}
