//! BitPosition struct.

use crate::prelude::*;

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
pub struct BitPosition {
    /// Bitmask for the positions of all Kings on the board.
    kings: u64,
    /// Bitmask for the positions of all Queens on the board.
    queens: u64,
    /// Bitmask for the positions of all Rooks on the board.
    rooks: u64,
    /// Bitmask for the positions of all Bishops on the board.
    bishops: u64,
    /// Bitmask for the positions of all Knights on the board.
    knights: u64,
    /// Bitmask for the positions of all Pawns on the board.
    pawns: u64,
    /// Bitmask for the White pieces on the board.
    white: u64,
    /// Bitmask for the Black pieces on the board.
    black: u64,
}

impl BitPosition {
    /// New [`Position`] given a [`ChessVariant`].
    pub fn new(chess_variant: Variant) -> BitPosition {
        match chess_variant {
            Variant::Empty => BitPosition::default(),
            Variant::StandardChess => BitPosition {
                kings: 0x1000000000000010,
                queens: 0x2000000000000020,
                rooks: 0x8100000000000081,
                bishops: 0x2400000000000024,
                knights: 0x4200000000000042,
                pawns: 0xFF00000000FF00,
                white: 0xFFFF,
                black: 0xFFFF000000000000,
            },
        }
    }
}

impl Position for BitPosition {
    /// Get the piece at the given coordinate.
    fn get_piece(&self, coord: Coord) -> Option<Piece> {
        None
    }

    /// Get the available [`Piece`]s for a [`Color`].
    fn get_pieces(&self, piece: Piece) -> Vec<Piece> {
        Vec::<Piece>::new()
    }

    /// Put a piece onto the board. Return any piece on the given square if it's occupied
    /// already.
    fn put_piece(&mut self, piece: Option<Piece>, coord: Coord) -> Option<Piece> {
        None
    }

    /// Get all [`Piece`]s attacking a given coordinate.
    fn get_pieces_attacking_coord(
        &self,
        board: &EasyBoard,
        piece: Piece,
        coord: Coord,
    ) -> Vec<Piece> {
        Vec::<Piece>::new()
    }
}
