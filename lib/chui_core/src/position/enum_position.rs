//! `EnumPosition` struct.

use std::fmt;
use std::ops::{Deref, DerefMut};

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
/// Piece Enum
pub enum PieceEnum {
    /// White King
    WhiteKing,
    /// White Queen
    WhiteQueen,
    /// White Rook
    WhiteRook,
    /// White Bishop
    WhiteBishop,
    /// White Knight
    WhiteKnight,
    /// White Pawn
    WhitePawn,
    /// Black King
    BlackKing,
    /// Black Queen
    BlackQueen,
    /// Black Rook
    BlackRook,
    /// Black Bishop
    BlackBishop,
    /// Black Knight
    BlackKnight,
    /// Black Pawn
    BlackPawn,
}

impl PieceEnum {
    /// Get piece kind
    pub const fn get_piece(&self) -> PieceKind {
        match self {
            PieceEnum::WhiteKing | PieceEnum::BlackKing => PieceKind::King,
            PieceEnum::WhiteQueen | PieceEnum::BlackQueen => PieceKind::Queen,
            PieceEnum::WhiteRook | PieceEnum::BlackRook => PieceKind::Rook,
            PieceEnum::WhiteBishop | PieceEnum::BlackBishop => PieceKind::Bishop,
            PieceEnum::WhiteKnight | PieceEnum::BlackKnight => PieceKind::Knight,
            PieceEnum::WhitePawn | PieceEnum::BlackPawn => PieceKind::Pawn,
        }
    }

    /// Get piece color
    pub const fn get_color(&self) -> Color {
        match self {
            PieceEnum::WhiteKing
            | PieceEnum::WhiteQueen
            | PieceEnum::WhiteRook
            | PieceEnum::WhiteBishop
            | PieceEnum::WhiteKnight
            | PieceEnum::WhitePawn => Color::White,
            PieceEnum::BlackKing
            | PieceEnum::BlackQueen
            | PieceEnum::BlackRook
            | PieceEnum::BlackBishop
            | PieceEnum::BlackKnight
            | PieceEnum::BlackPawn => Color::Black,
        }
    }
}

/// Displays the position for White.
impl fmt::Display for PieceEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            Piece::new(self.get_piece(), self.get_color(), Coord::zero())
        )
    }
}

impl From<Piece> for PieceEnum {
    fn from(piece: Piece) -> PieceEnum {
        match (piece.get_kind(), piece.get_color()) {
            // White pieces
            (PieceKind::King, Color::White) => PieceEnum::WhiteKing,
            (PieceKind::Queen, Color::White) => PieceEnum::WhiteQueen,
            (PieceKind::Rook, Color::White) => PieceEnum::WhiteRook,
            (PieceKind::Bishop, Color::White) => PieceEnum::WhiteBishop,
            (PieceKind::Knight, Color::White) => PieceEnum::WhiteKnight,
            (PieceKind::Pawn, Color::White) => PieceEnum::WhitePawn,
            // Black pieces
            (PieceKind::King, Color::Black) => PieceEnum::BlackKing,
            (PieceKind::Queen, Color::Black) => PieceEnum::BlackQueen,
            (PieceKind::Rook, Color::Black) => PieceEnum::BlackRook,
            (PieceKind::Bishop, Color::Black) => PieceEnum::BlackBishop,
            (PieceKind::Knight, Color::Black) => PieceEnum::BlackKnight,
            (PieceKind::Pawn, Color::Black) => PieceEnum::BlackPawn,
        }
    }
}

impl From<PieceEnum> for Piece {
    fn from(piece_enum: PieceEnum) -> Self {
        Piece::new(
            piece_enum.get_piece(),
            piece_enum.get_color(),
            Coord::zero(),
        )
    }
}

/// `EnumArray` represents an array of `Option<PieceEnum>`s according to the defined number
/// of `RANKS` and `FILES`.
pub type EnumArray = [Option<PieceEnum>; (FILES * RANKS) as usize];

/// Position of the chessboard represented by a 2D array of `Option<Piece>`s.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnumPosition(EnumArray);

impl EnumPosition {
    /// New `EnumPosition` given a chess Variant.
    pub fn new(chess_variant: Variant) -> EnumPosition {
        match chess_variant {
            Variant::Empty => EnumPosition::default(),
            Variant::StandardChess => EnumPosition::new_standard_chess(),
        }
    }

    /// New standard chess setup.
    ///
    /// # Panics
    ///
    /// This method does not panic because of the use of known constants.
    pub const fn new_standard_chess() -> EnumPosition {
        EnumPosition([
            // Rank 1
            Some(PieceEnum::WhiteRook),
            Some(PieceEnum::WhiteKnight),
            Some(PieceEnum::WhiteBishop),
            Some(PieceEnum::WhiteQueen),
            Some(PieceEnum::WhiteKing),
            Some(PieceEnum::WhiteBishop),
            Some(PieceEnum::WhiteKnight),
            Some(PieceEnum::WhiteRook),
            // Rank 2
            Some(PieceEnum::WhitePawn),
            Some(PieceEnum::WhitePawn),
            Some(PieceEnum::WhitePawn),
            Some(PieceEnum::WhitePawn),
            Some(PieceEnum::WhitePawn),
            Some(PieceEnum::WhitePawn),
            Some(PieceEnum::WhitePawn),
            Some(PieceEnum::WhitePawn),
            // Rank 3
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            // Rank 4
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            // Rank 5
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            // Rank 6
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            // Rank 7
            Some(PieceEnum::BlackPawn),
            Some(PieceEnum::BlackPawn),
            Some(PieceEnum::BlackPawn),
            Some(PieceEnum::BlackPawn),
            Some(PieceEnum::BlackPawn),
            Some(PieceEnum::BlackPawn),
            Some(PieceEnum::BlackPawn),
            Some(PieceEnum::BlackPawn),
            // Rank 8
            Some(PieceEnum::BlackRook),
            Some(PieceEnum::BlackKnight),
            Some(PieceEnum::BlackBishop),
            Some(PieceEnum::BlackQueen),
            Some(PieceEnum::BlackKing),
            Some(PieceEnum::BlackBishop),
            Some(PieceEnum::BlackKnight),
            Some(PieceEnum::BlackRook),
        ])
    }
}

impl Position for EnumPosition {
    /// Get the piece at the given coordinate.
    fn get_piece(&self, coord: Option<Coord>) -> Option<Piece> {
        // If there's no coordinate, there's no piece.
        coord?;
        let idx = coord.unwrap().get_index() as usize;
        let enum_piece = self[idx];
        enum_piece.map(|e_p| Piece::new(e_p.get_piece(), e_p.get_color(), coord.unwrap()))
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
    fn put_piece(&mut self, piece: Option<Piece>, coord: Option<Coord>) -> Option<Piece> {
        // If there's no coordinate, there's no piece to put.
        coord?;
        let idx = coord.unwrap().get_index() as usize;
        let return_piece = self.get_piece(coord);

        if let Some(mut piece) = piece {
            piece.set_coord(coord);
            self[idx] = Some(piece.into());
        } else {
            self[idx] = None;
        }

        return_piece
    }

    /// Get all [`Piece`]s attacking a given coordinate.
    fn get_pieces_attacking_coord(
        &self,
        _board: &Board,
        _piece: Piece,
        _coord: Option<Coord>,
    ) -> Vec<Piece> {
        Vec::<Piece>::new()
        // self.iter()
        //     .filter_map(|o_p| {
        //         // o_p.filter(|p| p.get_color() != color && p.get_move_coords(self).contains(&coord))
        //         o_p.filter(|&p| {
        //             let mut the_piece: Piece = p.into();
        //             the_piece.set_coord(coord);
        //             the_piece.get_move_coords(board, Some(piece)).contains(&coord)
        //         })
        //     })
        //     .collect()
    }
}

impl Deref for EnumPosition {
    type Target = EnumArray;

    fn deref(&self) -> &EnumArray {
        &self.0
    }
}

impl DerefMut for EnumPosition {
    fn deref_mut(&mut self) -> &mut EnumArray {
        &mut self.0
    }
}

impl Iterator for EnumPosition {
    type Item = Piece;

    fn next(&mut self) -> Option<Self::Item> {
        // self.0.iter().flatten().next().unwrap_or_default(None).to_owned()
        None
    }
}

/// Displays the position for White.
impl fmt::Display for EnumPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.white_to_string())
    }
}

impl Default for EnumPosition {
    fn default() -> EnumPosition {
        EnumPosition([None; (FILES * RANKS) as usize])
    }
}
