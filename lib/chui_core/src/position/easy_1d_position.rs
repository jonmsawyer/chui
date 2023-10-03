//! `Easy1DPosition` struct.

use std::fmt;
use std::ops::{Deref, DerefMut};

use crate::prelude::{coord::*, *};

/// `Array1D` represents a 1D array of `Option<Piece>`s according to the defined number of `RANKS`
/// and `FILES`.
pub type Array1D = [Option<Piece>; (FILES * RANKS) as usize];

/// Position of the chessboard represented by a 2D array of `Option<Piece>`s.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Easy1DPosition(Array1D);

impl Easy1DPosition {
    /// New `Easy1DPosition` given a chess Variant.
    pub fn new(chess_variant: Variant) -> Easy1DPosition {
        match chess_variant {
            Variant::Empty => Easy1DPosition::default(),
            Variant::StandardChess => Easy1DPosition::new_standard_chess(),
        }
    }

    /// New standard chess setup.
    ///
    /// # Panics
    ///
    /// This method does not panic because of the use of known constants.
    pub fn new_standard_chess() -> Easy1DPosition {
        Easy1DPosition([
            // rank 1
            Some(Piece::new(
                PieceKind::Rook,
                Color::White,
                Coord::try_from(A1).unwrap(),
            )),
            Some(Piece::new(
                PieceKind::Knight,
                Color::White,
                Coord::try_from(B1).unwrap(),
            )),
            Some(Piece::new(
                PieceKind::Bishop,
                Color::White,
                Coord::try_from(C1).unwrap(),
            )),
            Some(Piece::new(
                PieceKind::Queen,
                Color::White,
                Coord::try_from(D1).unwrap(),
            )),
            Some(Piece::new(
                PieceKind::King,
                Color::White,
                Coord::try_from(E1).unwrap(),
            )),
            Some(Piece::new(
                PieceKind::Bishop,
                Color::White,
                Coord::try_from(F1).unwrap(),
            )),
            Some(Piece::new(
                PieceKind::Knight,
                Color::White,
                Coord::try_from(G1).unwrap(),
            )),
            Some(Piece::new(
                PieceKind::Rook,
                Color::White,
                Coord::try_from(H1).unwrap(),
            )),
            // rank 2
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
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None, // rank 3
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None, // rank 4
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None, // rank 5
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None, // rank 6
            // rank 7
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
            // rank 8
            Some(Piece::new(
                PieceKind::Rook,
                Color::Black,
                Coord::try_from(A8).unwrap(),
            )),
            Some(Piece::new(
                PieceKind::Knight,
                Color::Black,
                Coord::try_from(B8).unwrap(),
            )),
            Some(Piece::new(
                PieceKind::Bishop,
                Color::Black,
                Coord::try_from(C8).unwrap(),
            )),
            Some(Piece::new(
                PieceKind::Queen,
                Color::Black,
                Coord::try_from(D8).unwrap(),
            )),
            Some(Piece::new(
                PieceKind::King,
                Color::Black,
                Coord::try_from(E8).unwrap(),
            )),
            Some(Piece::new(
                PieceKind::Bishop,
                Color::Black,
                Coord::try_from(F8).unwrap(),
            )),
            Some(Piece::new(
                PieceKind::Knight,
                Color::Black,
                Coord::try_from(G8).unwrap(),
            )),
            Some(Piece::new(
                PieceKind::Rook,
                Color::Black,
                Coord::try_from(H8).unwrap(),
            )),
        ])
    }

    /// Set the Coordinates for all [`Piece`]s.
    ///
    /// # Panics
    ///
    /// This method should never panic due to the iterated index never being out of range.
    ///
    /// TODO: Unused method.
    pub fn set_coords(&mut self) {
        self.iter_mut().enumerate().for_each(|(idx, piece)| {
            if let Some(piece) = piece {
                piece.set_coord(Coord::try_from(idx as u8).unwrap()); // .unwrap() should never panic.
            }
        });
    }
}

impl Position for Easy1DPosition {
    /// Get the piece at the given coordinate.
    fn get_piece(&self, coord: Coord) -> Option<Piece> {
        self[coord.get_index() as usize]
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
            self[coord.get_index() as usize] = Some(piece);
        } else {
            self[coord.get_index() as usize] = None;
        }

        return_piece
    }

    /// Get all [`Piece`]s attacking a given coordinate.
    fn get_pieces_attacking_coord(&self, board: &Board, piece: Piece, coord: Coord) -> Vec<Piece> {
        self.iter()
            .filter_map(|o_p| {
                // o_p.filter(|p| p.get_color() != color && p.get_move_coords(self).contains(&coord))
                o_p.filter(|p| p.get_move_coords(board, Some(piece)).contains(&coord))
            })
            .collect()
    }
}

impl Deref for Easy1DPosition {
    type Target = Array1D;

    fn deref(&self) -> &Array1D {
        &self.0
    }
}

impl DerefMut for Easy1DPosition {
    fn deref_mut(&mut self) -> &mut Array1D {
        &mut self.0
    }
}

impl Iterator for Easy1DPosition {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        // self.0.iter().flatten().next().unwrap_or_default(None).to_owned()
        None
    }
}

/// Displays the position for White.
impl fmt::Display for Easy1DPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.white_to_string())
    }
}

impl Default for Easy1DPosition {
    fn default() -> Easy1DPosition {
        Easy1DPosition([None; (FILES * RANKS) as usize])
    }
}
