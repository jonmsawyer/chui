//! `ArrayArrayBitPosition` struct.

// use std::array;
use std::fmt;
use std::ops::{Deref, DerefMut};

use crate::prelude::*;

/// Type define a [u64; 8] array.
pub type BitmaskArray = [u64; 8];

/// Position of the chessboard represented by u64 bitmasks. Each piece kind has a mask and finally
/// a mask for each color. This is the smallest pre-allocated data structure possible to represent
/// any given (standard or non-standard) chess position. Least significant bit (LSB) represents
/// `a1` and most significant bit (MSB) represents `h8`, with increasing files then increasing
/// ranks. In other words, the higher the particular mask value the farther up the board the piece
/// or position is from White's perspective.
///
/// ## Examples
///
/// ### Kings
///
/// Bitmask for all Kings on the board given the Standard Chess variation:
///
///        h8            a8
///     MSB 0 0 0 1 0 0 0 0
///         0 0 0 0 0 0 0 0
///         0 0 0 0 0 0 0 0
///         0 0 0 0 0 0 0 0
///         0 0 0 0 0 0 0 0
///         0 0 0 0 0 0 0 0
///         0 0 0 0 0 0 0 0
///         0 0 0 1 0 0 0 0 LSB
///        h1             a1
///
/// * The above bitmask in decimal: `115_292_1504_606_846_992`.
/// * The above bitmask in hexidecimal: `0x10_00_00_00_00_00_00_10`.
/// * The above bitmask in octal: `0o100_000_000_000_000_000_020`.
/// * The above bitmask in binary: `0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00010000`
///
/// #### Pawns
///
/// Bitmask for all Pawns on the board given the Standard Chess variation:
///
///        h8            a8
///     MSB 0 0 0 0 0 0 0 0
///         1 1 1 1 1 1 1 1
///         0 0 0 0 0 0 0 0
///         0 0 0 0 0 0 0 0
///         0 0 0 0 0 0 0 0
///         0 0 0 0 0 0 0 0
///         1 1 1 1 1 1 1 1
///         0 0 0 0 0 0 0 0 LSB
///        h1             a1
///
/// * The above bitmask in decimal: `71_776_119_061_282_560`.
/// * The above bitmask in hexidecimal: `0x00_FF_00_00_00_00_FF_00`.
/// * The above bitmask in octal: `0o3_770_000_000_000_177_400`.
/// * The above bitmask in binary: `0b00000000_11111111_00000000_00000000_00000000_00000000_11111111_00000000`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayBitPosition(BitmaskArray);

impl ArrayBitPosition {
    /// New [`Position`] given a [`ChessVariant`].
    pub fn new(chess_variant: Variant) -> ArrayBitPosition {
        match chess_variant {
            Variant::Empty => ArrayBitPosition::default(),
            Variant::StandardChess => ArrayBitPosition([
                0x1000000000000010, // Kings
                0x800000000000008,  // Queens
                0x8100000000000081, // Rooks
                0x2400000000000024, // Bishops
                0x4200000000000042, // Knights
                0xFF00000000FF00,   // Pawns
                0xFFFF,             // White
                0xFFFF000000000000, // Black
            ]),
        }
    }
}

impl ArrayBitPosition {
    /// Get the piece kind from an index.
    ///
    /// # Errors
    ///
    /// Errors when `idx` (index) parameter is out of  range.
    #[allow(clippy::unused_self)]
    fn get_piece_kind_from_idx(&self, idx: usize) -> ChuiResult<PieceKind> {
        match idx {
            0 => Ok(PieceKind::King),
            1 => Ok(PieceKind::Queen),
            2 => Ok(PieceKind::Rook),
            3 => Ok(PieceKind::Bishop),
            4 => Ok(PieceKind::Knight),
            5 => Ok(PieceKind::Pawn),
            _ => Err(ChuiError::IndexOutOfRange(format!(
                "Index `{idx}` is out of range (0..=5)"
            ))),
        }
    }

    /// Get the piece kind from an index.
    ///
    /// # Errors
    ///
    /// Errors when `idx` (index) parameter is out of  range.
    #[allow(clippy::unused_self)]
    fn get_piece_color_from_idx(&self, idx: usize) -> ChuiResult<Color> {
        match idx {
            6 => Ok(Color::White),
            7 => Ok(Color::Black),
            _ => Err(ChuiError::IndexOutOfRange(format!(
                "Index `{idx}` is out of range (0..=1) or (6..=7)"
            ))),
        }
    }

    /// Get the piece kind from an index.
    ///
    /// # Errors
    ///
    /// Errors when `idx` (index) parameter is out of  range.
    #[allow(clippy::unused_self)]
    const fn get_idx_from_piece_kind(&self, piece_kind: PieceKind) -> usize {
        match piece_kind {
            PieceKind::King => 0,
            PieceKind::Queen => 1,
            PieceKind::Rook => 2,
            PieceKind::Bishop => 3,
            PieceKind::Knight => 4,
            PieceKind::Pawn => 5,
        }
    }

    /// Get the piece kind from an index.
    ///
    /// # Errors
    ///
    /// Errors when `idx` (index) parameter is out of  range.
    #[allow(clippy::unused_self)]
    const fn get_idx_from_piece_color(&self, color: Color) -> usize {
        match color {
            Color::White => 6,
            Color::Black => 7,
        }
    }

    /// Get the piece kind from a bitmask.
    fn get_piece_kind_from_bitmask(&self, bitmask: u64) -> ChuiResult<PieceKind> {
        // Calculate the `PieceKind` of the new `Piece`.
        for (idx, piece_mask) in self.0.iter().enumerate().take(6) {
            // println!("ArrayBitPosition.get_piece() idx for piece kind: {idx}");
            if bitmask & *piece_mask > 0 {
                return self.get_piece_kind_from_idx(idx);
            }
        }

        Err(ChuiError::InvalidPiece(
            "A piece kind could not be generated from the input bitmask.".to_string(),
        ))
    }

    /// Get the piece color from a bitmask.
    fn get_piece_color_from_bitmask(&self, bitmask: u64) -> ChuiResult<Color> {
        // Calculate the `Color` of the new `Piece`.
        for (idx, color_mask) in self.0.iter().enumerate().skip(6) {
            // println!("ArrayBitPosition.get_piece() idx for piece color: {idx}");
            if bitmask & *color_mask > 0 {
                return self.get_piece_color_from_idx(idx);
            }
        }

        Err(ChuiError::InvalidPiece(
            "A piece color could not be generated from the input bitmask.".to_string(),
        ))
    }

    /// Set the piece according to its bitmask.
    fn set_piece_from_bitmask(&mut self, piece: Option<Piece>, bitmask: u64) -> ChuiResult<()> {
        if let Some(piece) = piece {
            let piece_kind_idx = self.get_idx_from_piece_kind(piece.get_kind());
            let piece_color_idx = self.get_idx_from_piece_color(piece.get_color());

            for (idx, piece_mask) in self.iter_mut().enumerate().take(6) {
                if idx == piece_kind_idx {
                    *piece_mask |= bitmask;
                } else if *piece_mask & bitmask > 0 {
                    *piece_mask ^= bitmask;
                }
            }

            for (idx, color_mask) in self.iter_mut().enumerate().skip(6) {
                if idx == piece_color_idx {
                    *color_mask |= bitmask;
                } else if *color_mask & bitmask > 0 {
                    *color_mask ^= bitmask;
                }
            }
        } else {
            for mask in self.iter_mut() {
                if *mask & bitmask > 0 {
                    *mask ^= bitmask;
                }
            }
        }

        Ok(())
    }
}

impl Position for ArrayBitPosition {
    /// Get the piece at the given coordinate.
    fn get_piece(&self, coord: Coord) -> Option<Piece> {
        let piece_kind: PieceKind;
        let piece_color: Color;
        let idx = coord.get_index();
        let bitmask: u64 = 1 << idx;

        if let Ok(kind) = self.get_piece_kind_from_bitmask(bitmask) {
            piece_kind = kind;
        } else {
            return None;
        }

        if let Ok(color) = self.get_piece_color_from_bitmask(bitmask) {
            piece_color = color;
        } else {
            return None;
        }

        // We have a valid piece that can be constructed.
        Some(Piece::new(piece_kind, piece_color, coord))
    }

    // /// Get the available [`Piece`]s for a [`Color`].
    // fn get_pieces(&self, piece: Piece) -> Vec<Piece> {
    //     let mut pieces = Vec::<Piece>::new();
    //     let mut bitmask: u64 = 1;
    //     let mut idx: u8 = 0;

    //     match piece.get_color() {
    //         Color::White => {
    //             //
    //         },
    //         Color::Black => {
    //             //
    //         }
    //     }
    //     pieces
    // }

    /// Put a piece onto the board. Return any piece on the given square if it's occupied
    /// already.
    fn put_piece(&mut self, piece: Option<Piece>, coord: Coord) -> Option<Piece> {
        let ret_piece = self.get_piece(coord);
        let idx = coord.get_index();
        let bitmask: u64 = 1 << idx;

        self.set_piece_from_bitmask(piece, bitmask).ok();

        ret_piece
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
}

impl Deref for ArrayBitPosition {
    type Target = BitmaskArray;

    fn deref(&self) -> &BitmaskArray {
        &self.0
    }
}

impl DerefMut for ArrayBitPosition {
    fn deref_mut(&mut self) -> &mut BitmaskArray {
        &mut self.0
    }
}

// impl Iterator for ArrayBitPosition {
//     type Item = u64;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.iter().next().map_or(None, |item| Some(*item))
//     }
// }

/// Displays the position for White.
impl fmt::Display for ArrayBitPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.white_to_string())
    }
}
