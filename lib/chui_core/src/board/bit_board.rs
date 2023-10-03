//! Bit Board module.

// use std::ops::Index;

// use nonmax::NonMaxU8;

// use crate::constants::bitmask::{FILES, RANKS};

use crate::prelude::*;

/// This struct represents the chessboard. Has a field called `board` which
/// references an 8x8 board. Has a field called `en_passant` which represents the en passant
/// target square.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitBoard {
    /// Represents an 8x8 chessboard using bits for position.
    position: BitPosition,

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

impl BitBoard {
    //
    // Constructors.
    //

    /// Return a new [`Board`] given a [`ChessVariant`].
    pub fn new(variant: Variant) -> BitBoard {
        let default = BitBoard {
            position: BitPosition::new(Variant::StandardChess),
            white_can_castle_kingside: true,
            white_can_castle_queenside: true,
            black_can_castle_kingside: true,
            black_can_castle_queenside: true,
            en_passant_target_square: None,
            true_enpassant_target_square: None,
            en_passant_target_piece: None,
        };

        match variant {
            Variant::StandardChess => default,
            Variant::Empty => BitBoard {
                position: BitPosition::new(Variant::Empty),
                ..default
            },
        }
    }
}

impl Default for BitBoard {
    fn default() -> Self {
        BitBoard::new(Variant::StandardChess)
    }
}
