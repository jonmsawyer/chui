//! Board module.

// use std::ops::Index;

// use nonmax::NonMaxU8;

#[allow(clippy::wildcard_imports)]
use crate::constants::*;
use crate::{ChuiError, ChuiResult, Color, Coord, Move, Piece, PieceKind, FILES, RANKS};

mod tests;

/// The various chess variants available in Chui.
#[derive(Debug, Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ChessVariant {
    /// Standard Chess is the default chess variant. Used in all tournaments
    /// and official gameplay.
    #[default]
    StandardChess,

    /// Empty chessboard.
    Empty,
    //Chess960,
}

/// This struct represents the chessboard. Has a field called `board` which
/// references an 8x8 board. Has a field called `en_passant` which represents the en passant
/// target square.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board {
    /// Represents an 8x8 chessboard using nested arrays.
    board: [[Option<Piece>; FILES as usize]; RANKS as usize],

    /// Can white castle on the king side?
    pub white_can_castle_kingside: bool,

    /// Can white castle on the queen side?
    pub white_can_castle_queenside: bool,

    /// Can black castle on the king side?
    pub black_can_castle_kingside: bool,

    /// Can black castle on the queen side?
    pub black_can_castle_queenside: bool,

    /// When a pawn is moved, the en passant target square is
    /// noted, even if there's no en passant move possible. This
    /// comes from the FEN layout of the game.
    en_passant_target_square: Option<Coord>,

    /// When a pawn is moved, the en passant target square is
    /// noted, only if there's an en passant move possible. This
    /// comes from the X-FEN layout of the game.
    true_enpassant_target_square: Option<Coord>,

    /// Represents the en passant target piece (pawn).
    en_passant_target_piece: Option<Piece>,
}

impl Board {
    //
    // Constructors.
    //

    /// Return a new [`Board`] given a [`ChessVariant`].
    pub fn new(variant: ChessVariant) -> Board {
        let default = Board {
            board: Board::new_standard_chess(),
            white_can_castle_kingside: true,
            white_can_castle_queenside: true,
            black_can_castle_kingside: true,
            black_can_castle_queenside: true,
            en_passant_target_square: None,
            true_enpassant_target_square: None,
            en_passant_target_piece: None,
        };

        match variant {
            ChessVariant::StandardChess => default,
            ChessVariant::Empty => Board {
                board: Board::new_empty_board(),
                ..default
            },
        }
    }

    /// New standard chess setup.
    ///
    /// # Panics
    ///
    /// This method does not panic because of the use of known constants.
    pub fn new_standard_chess() -> [[Option<Piece>; FILES as usize]; RANKS as usize] {
        [
            // rank 1
            Board::standard_row_of_pieces(Color::White, 0).unwrap(),
            // rank 2
            [
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::White,
                    Coord::try_from(A2).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::White,
                    Coord::try_from(B2).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::White,
                    Coord::try_from(C2).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::White,
                    Coord::try_from(D2).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::White,
                    Coord::try_from(E2).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::White,
                    Coord::try_from(F2).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::White,
                    Coord::try_from(G2).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::White,
                    Coord::try_from(H2).unwrap(),
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
                    Coord::try_from(A7).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::Black,
                    Coord::try_from(B7).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::Black,
                    Coord::try_from(C7).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::Black,
                    Coord::try_from(D7).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::Black,
                    Coord::try_from(E7).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::Black,
                    Coord::try_from(F7).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::Black,
                    Coord::try_from(G7).unwrap(),
                )),
                Some(Piece::new(
                    PieceKind::Pawn,
                    Color::Black,
                    Coord::try_from(H7).unwrap(),
                )),
            ],
            // rank 8
            Board::standard_row_of_pieces(Color::Black, 7).unwrap(),
        ]
    }

    /// Return an empty board.
    pub const fn new_empty_board() -> [[Option<Piece>; FILES as usize]; RANKS as usize] {
        [[None; FILES as usize]; RANKS as usize]
    }

    //
    // Conditionals.
    //

    /// Apply the passed in move onto the chessboard.
    ///
    /// # Errors
    ///
    /// Errors if the piece we're moving is `None`.
    ///
    /// # Panics
    ///
    /// Panics when the `move_obj`'s piece is None after checking that it is Some.
    pub fn apply_move(&mut self, move_obj: &Move) -> ChuiResult<Option<Piece>> {
        if move_obj.get_piece().is_none() {
            return Err(ChuiError::InvalidMove(
                "No piece to apply move.".to_string(),
            ));
        }

        let pieces = self.get_pieces(move_obj.get_piece().unwrap());

        // println!("Pieces: {:?}", pieces);

        let mut pieces_can_move = Vec::<Piece>::new();

        for piece in pieces.iter() {
            if piece.get_move_coords(self).iter().any(|&coord| {
                coord.get_file() == move_obj.to_coord.get_file()
                    && coord.get_rank() == move_obj.to_coord.get_rank()
            }) {
                pieces_can_move.push(*piece);
            }
        }

        // println!("Pieces can move: {:?}", pieces_can_move);

        let (file, rank) = move_obj.to_coord.to_u8_index();

        if pieces_can_move.is_empty() {
            Err(ChuiError::InvalidMove(format!(
                "No {} can move to target square {}",
                move_obj.get_piece().unwrap(),
                move_obj.to_coord
            )))
        } else if pieces_can_move.len() == 1 {
            let piece = pieces_can_move.get(0).unwrap();
            self.replace_piece(*piece, move_obj)
        } else {
            Err(ChuiError::InvalidMove(format!(
                "Ambiguous move. More than one piece can move to target square {}{}",
                file, rank
            )))
        }
    }

    /// Replace the given piece from one square to another.
    ///
    /// # Errors
    ///
    /// This method does not error.
    pub fn replace_piece(
        &mut self,
        mut piece_from: Piece,
        move_obj: &Move,
    ) -> ChuiResult<Option<Piece>> {
        self.take_piece(piece_from.get_coord());
        piece_from.set_coord(move_obj.to_coord);
        Ok(self.put_piece(Some(piece_from), move_obj.to_coord))
    }

    //
    // Getters.
    //

    /// Get a refereance to the board.
    pub const fn get_board(&self) -> &[[Option<Piece>; FILES as usize]; RANKS as usize] {
        &self.board
    }

    /// Get the piece at the given coordinate.
    pub const fn get_piece(&self, coord: Coord) -> Option<Piece> {
        self.board[coord.get_rank() as usize][coord.get_file() as usize]
    }

    /// Get the available [`Piece`]s for a [`Color`].
    pub fn get_pieces(&self, piece: Piece) -> Vec<Piece> {
        self.board
            .iter()
            .flatten()
            .filter_map(|o_p| o_p.filter(|p| p.is_same_piece(piece)))
            .collect()
    }

    /// Get the en passant target square coordinate.
    pub const fn get_en_passant_coord(&self) -> Option<Coord> {
        self.en_passant_target_square
    }

    /// Get the en passant target piece.
    pub const fn get_en_passant_piece(&self) -> Option<Piece> {
        self.en_passant_target_piece
    }

    /// Get the en passant target square and piece.
    pub const fn get_en_passant(&self) -> (Option<Coord>, Option<Piece>) {
        (self.get_en_passant_coord(), self.get_en_passant_piece())
    }

    //
    // Setters.
    //

    /// Set the en passant target square coordinate.
    pub fn set_en_passant_coord(&mut self, coord: Option<Coord>) {
        self.en_passant_target_square = coord;
        self.true_enpassant_target_square = coord;
    }

    /// Set the en passant target square piece.
    pub fn set_en_passant_piece(&mut self, piece: Option<Piece>) {
        self.en_passant_target_piece = piece;
    }

    /// Set both the en passant target square and piece.
    pub fn set_en_passant(&mut self, coord: Option<Coord>, piece: Option<Piece>) {
        self.set_en_passant_coord(coord);
        self.set_en_passant_piece(piece);
    }

    /// Set the Coordinates for all [`Piece`]s.
    ///
    /// # Panics
    ///
    /// This method should never panic due to the iterated index never being out of range.
    pub fn set_coords(&mut self) {
        self.board
            .iter_mut()
            .flatten()
            .enumerate()
            .for_each(|(idx, piece)| {
                if let Some(piece) = piece {
                    piece.set_coord(Coord::new_from_idx(idx as u8).unwrap()); // .unwrap() should never panic.
                }
            });
    }

    /// Take a piece off of the board.
    pub fn take_piece(&mut self, coord: Coord) -> Option<Piece> {
        self.put_piece(None, coord)
    }

    /// Put a piece onto the board. Return any piece on the given square if it's occupied
    /// already.
    pub fn put_piece(&mut self, piece: Option<Piece>, coord: Coord) -> Option<Piece> {
        let return_piece = self.get_piece(coord);
        self.board[coord.get_rank() as usize][coord.get_file() as usize] = piece;
        return_piece
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

        // Trim the move coordinates which are attacking the King.
        let mut coords: Vec<Coord> = coords
            .into_iter()
            .filter(|&c| {
                self.get_pieces_attacking_coord(piece.get_color(), c)
                    .is_empty()
            })
            .collect();

        // Add any valid castling coordinates
        match piece.get_color() {
            Color::White => {
                if self.white_can_castle_kingside {
                    let d1 = Coord::try_from(D1).unwrap();
                    let c1 = Coord::try_from(C1).unwrap();
                    let d1_pieces = self.get_pieces_attacking_coord(piece.get_color(), d1);
                    let c1_pieces = self.get_pieces_attacking_coord(piece.get_color(), c1);

                    if d1_pieces.is_empty() && c1_pieces.is_empty() {
                        coords.extend(vec![c1]);
                    }
                }

                if self.white_can_castle_queenside {
                    let f1 = Coord::try_from(F1).unwrap();
                    let g1 = Coord::try_from(G1).unwrap();
                    let f1_pieces = self.get_pieces_attacking_coord(piece.get_color(), f1);
                    let g1_pieces = self.get_pieces_attacking_coord(piece.get_color(), g1);

                    if f1_pieces.is_empty() && g1_pieces.is_empty() {
                        coords.extend(vec![g1]);
                    }
                }
            }
            Color::Black => {
                if self.black_can_castle_kingside {
                    let d8 = Coord::try_from(D8).unwrap();
                    let c8 = Coord::try_from(C8).unwrap();
                    let d8_pieces = self.get_pieces_attacking_coord(piece.get_color(), d8);
                    let c8_pieces = self.get_pieces_attacking_coord(piece.get_color(), c8);

                    if d8_pieces.is_empty() && c8_pieces.is_empty() {
                        coords.extend(vec![c8]);
                    }
                }

                if self.black_can_castle_queenside {
                    let f8 = Coord::try_from(F8).unwrap();
                    let g8 = Coord::try_from(G8).unwrap();
                    let f8_pieces = self.get_pieces_attacking_coord(piece.get_color(), f8);
                    let g8_pieces = self.get_pieces_attacking_coord(piece.get_color(), g8);

                    if f8_pieces.is_empty() && g8_pieces.is_empty() {
                        coords.extend(vec![g8]);
                    }
                }
            }
        }

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
    pub fn get_right_coords(&self, piece: &Piece) -> Vec<Coord> {
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
    pub fn get_bottom_coords(&self, piece: &Piece) -> Vec<Coord> {
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
    pub fn get_left_coords(&self, piece: &Piece) -> Vec<Coord> {
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
    pub fn get_top_left_coords(&self, piece: &Piece) -> Vec<Coord> {
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
    pub fn get_top_right_coords(&self, piece: &Piece) -> Vec<Coord> {
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
    pub fn get_bottom_right_coords(&self, piece: &Piece) -> Vec<Coord> {
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
    pub fn get_bottom_left_coords(&self, piece: &Piece) -> Vec<Coord> {
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

    /// Get all [`Piece`]s attacking a given coordinate.
    pub fn get_pieces_attacking_coord(&self, color: Color, coord: Coord) -> Vec<Piece> {
        self.board
            .iter()
            .flatten()
            .filter_map(|o_p| {
                o_p.filter(|p| p.get_color() != color && p.get_move_coords(self).contains(&coord))
            })
            .collect()
    }

    //
    // Utilities.
    //

    /// Produces a row (`[Option<Piece>; FILES]`) of pieces
    /// according their color.
    ///
    /// # Errors
    ///
    /// Returns a [`ChuiError`] result if `rank_idx` is out of range.
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
            if c_string.is_empty() {
                c_string.push_str(format!("{}", c).as_str());
            } else {
                c_string.push_str(format!(", {}", c).as_str());
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

impl Default for Board {
    fn default() -> Self {
        Board::new(ChessVariant::StandardChess)
    }
}
