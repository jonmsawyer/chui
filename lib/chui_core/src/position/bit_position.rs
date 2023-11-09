//! `BitPosition` struct.

use std::fmt;

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
                queens: 0x800000000000008,
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
    fn get_piece(&self, coord: Option<Coord>) -> Option<Piece> {
        // If there's no coordinate, there's no piece.
        coord?;
        let coord: Coord = coord.unwrap();
        let piece_kind: PieceKind;
        let color: Color;
        let idx: u8 = coord.get_index();
        let mut bitmask: u64 = 1;

        // Adjust the bitmask according to the zero-based index given in `coord.get_index()`.
        // If the index is zero, shift nothing.
        bitmask <<= idx;

        // Calculate the `PieceKind`.
        if bitmask & self.kings > 0 {
            piece_kind = PieceKind::King;
        } else if bitmask & self.queens > 0 {
            piece_kind = PieceKind::Queen;
        } else if bitmask & self.rooks > 0 {
            piece_kind = PieceKind::Rook;
        } else if bitmask & self.bishops > 0 {
            piece_kind = PieceKind::Bishop;
        } else if bitmask & self.knights > 0 {
            piece_kind = PieceKind::Knight;
        } else if bitmask & self.pawns > 0 {
            piece_kind = PieceKind::Pawn;
        } else {
            return None;
        }

        // Calculate the `Color`.
        if bitmask & self.white > 0 {
            color = Color::White;
        } else if bitmask & self.black > 0 {
            color = Color::Black;
        } else {
            return None;
        }

        // We have a valid piece that can be constructed.
        Some(Piece::new(piece_kind, color, coord))
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
    ///
    /// TODO: Change this method's return type to ChuiResult<Option<Piece>>.
    fn put_piece(&mut self, piece: Option<Piece>, coord: Option<Coord>) -> Option<Piece> {
        // If there's no coordinate, the piece cannot be placed.
        coord?;
        let ret_piece: Option<Piece> = self.get_piece(coord);
        let idx: u8 = coord.unwrap().get_index();
        let bitmask: u64 = 1 << idx;

        if let Some(piece) = piece {
            match piece.get_kind() {
                PieceKind::King => {
                    if bitmask & self.kings == 0 {
                        self.kings |= bitmask;
                    }
                }
                PieceKind::Queen => {
                    if bitmask & self.queens == 0 {
                        self.queens |= bitmask;
                    }
                }
                PieceKind::Rook => {
                    if bitmask & self.rooks == 0 {
                        self.rooks |= bitmask;
                    }
                }
                PieceKind::Bishop => {
                    if bitmask & self.bishops == 0 {
                        self.kings |= bitmask;
                    }
                }
                PieceKind::Knight => {
                    if bitmask & self.knights == 0 {
                        self.knights |= bitmask;
                    }
                }
                PieceKind::Pawn => {
                    if bitmask & self.pawns == 0 {
                        self.pawns |= bitmask;
                    }
                }
            }

            match piece.get_color() {
                Color::White => {
                    if bitmask & self.white == 0 {
                        self.white |= bitmask;
                    }

                    if bitmask & self.black > 0 {
                        self.black ^= bitmask;
                    }
                }
                Color::Black => {
                    if bitmask & self.black == 0 {
                        self.black |= bitmask;
                    }

                    if bitmask & self.white > 0 {
                        self.white ^= bitmask;
                    }
                }
            }
        } else {
            if bitmask & self.kings > 0 {
                self.kings ^= bitmask;
            } else if bitmask & self.queens > 0 {
                self.queens ^= bitmask;
            } else if bitmask & self.rooks > 0 {
                self.rooks ^= bitmask;
            } else if bitmask & self.bishops > 0 {
                self.bishops ^= bitmask;
            } else if bitmask & self.knights > 0 {
                self.knights ^= bitmask;
            } else if bitmask & self.pawns > 0 {
                self.pawns ^= bitmask;
            }

            if bitmask & self.white > 0 {
                self.white ^= bitmask;
            } else if bitmask & self.black > 0 {
                self.black ^= bitmask;
            }
        }

        ret_piece
    }

    /// Get all [`Piece`]s attacking a given coordinate.
    fn get_pieces_attacking_coord(
        &self,
        _board: &Board,
        _piece: Piece,
        _coord: Option<Coord>,
    ) -> Vec<Piece> {
        Vec::<Piece>::new()
    }
}

/// Displays the position for White.
impl fmt::Display for BitPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.white_to_string())
    }
}
