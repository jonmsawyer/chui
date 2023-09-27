//! `EasyPosition` struct.

use std::fmt;
use std::ops::{Deref, DerefMut};

use crate::prelude::{coord::*, *};

/// `Array2D` represents a 2D array of `Option<Piece>`s according to the defined number of `RANKS`
/// and `FILES`.
pub type Array2D = [[Option<Piece>; FILES as usize]; RANKS as usize];

/// Position of the chessboard represented by a 2D array of `Option<Piece>`s.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EasyPosition(Array2D);

impl EasyPosition {
    /// New `EasyPosition` given a chess Variant.
    pub fn new(chess_variant: Variant) -> EasyPosition {
        match chess_variant {
            Variant::Empty => EasyPosition::default(),
            Variant::StandardChess => EasyPosition::new_standard_chess(),
        }
    }

    /// New standard chess setup.
    ///
    /// # Panics
    ///
    /// This method does not panic because of the use of known constants.
    pub fn new_standard_chess() -> EasyPosition {
        EasyPosition([
            // rank 1
            EasyPosition::standard_row_of_pieces(Color::White, 0).unwrap(),
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
            EasyPosition::standard_row_of_pieces(Color::Black, 7).unwrap(),
        ])
    }

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

    /// Set the Coordinates for all [`Piece`]s.
    ///
    /// # Panics
    ///
    /// This method should never panic due to the iterated index never being out of range.
    ///
    /// TODO: Unused method.
    pub fn set_coords(&mut self) {
        self.iter_mut()
            .flatten()
            .enumerate()
            .for_each(|(idx, piece)| {
                if let Some(piece) = piece {
                    piece.set_coord(Coord::try_from(idx as u8).unwrap()); // .unwrap() should never panic.
                }
            });
    }
}

impl Position for EasyPosition {
    /// Get the piece at the given coordinate.
    fn get_piece(&self, coord: Coord) -> Option<Piece> {
        self[coord.get_rank() as usize][coord.get_file() as usize]
    }

    // /// Get the available [`Piece`]s for a [`Color`].
    // fn get_pieces(&self, piece: Piece) -> Vec<Piece> {
    //     self.iter()
    //         .flatten()
    //         .filter_map(|o_p| o_p.filter(|p| p.is_same_piece(piece)))
    //         .collect()
    // }

    /// Put a piece onto the board. Return any piece on the given square if it's occupied
    /// already.
    fn put_piece(&mut self, piece: Option<Piece>, coord: Coord) -> Option<Piece> {
        let return_piece = self.get_piece(coord);

        if let Some(mut piece) = piece {
            piece.set_coord(coord);
            self[coord.get_rank() as usize][coord.get_file() as usize] = Some(piece);
        } else {
            self[coord.get_rank() as usize][coord.get_file() as usize] = None;
        }

        return_piece
    }

    /// Get all [`Piece`]s attacking a given coordinate.
    fn get_pieces_attacking_coord(&self, board: &Board, piece: Piece, coord: Coord) -> Vec<Piece> {
        self.iter()
            .flatten()
            .filter_map(|o_p| {
                // o_p.filter(|p| p.get_color() != color && p.get_move_coords(self).contains(&coord))
                o_p.filter(|p| p.get_move_coords(board, Some(piece)).contains(&coord))
            })
            .collect()
    }
}

impl Deref for EasyPosition {
    type Target = Array2D;

    fn deref(&self) -> &Array2D {
        &self.0
    }
}

impl DerefMut for EasyPosition {
    fn deref_mut(&mut self) -> &mut Array2D {
        &mut self.0
    }
}

impl Iterator for EasyPosition {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        // self.0.iter().flatten().next().unwrap_or_default(None).to_owned()
        None
    }
}

/// Displays the position for White.
impl fmt::Display for EasyPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.white_to_string())
    }
}
