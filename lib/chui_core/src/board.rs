//! Board module.

mod tests;

use crate::prelude::*;

/// This struct represents the chessboard. Has a field called `board` which
/// references an 8x8 board. Has a field called `en_passant` which represents the en passant
/// target square.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board {
    /// Represents an 8x8 chessboard.
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
    true_en_passant_target_square: Option<Coord>,

    /// Represents the en passant target piece (pawn).
    en_passant_target_piece: Option<Piece>,
}

impl Board {
    //
    // Constructors.
    //

    /// Return a new [`Board`] given a [`ChessVariant`].
    pub fn new(variant: Variant) -> Board {
        Board {
            position: BitPosition::new(variant),
            white_can_castle_kingside: true,
            white_can_castle_queenside: true,
            black_can_castle_kingside: true,
            black_can_castle_queenside: true,
            en_passant_target_square: None,
            true_en_passant_target_square: None,
            en_passant_target_piece: None,
        }
    }

    //
    // Conditionals.
    //

    // /// Apply the passed in move onto the chessboard.
    // ///
    // /// # Errors
    // ///
    // /// Errors if the piece we're moving is `None`.
    // ///
    // /// # Panics
    // ///
    // /// Panics when the `move_obj`'s piece is None after checking that it is Some.
    // pub fn apply_move(&mut self, move_obj: &Move) -> ChuiResult<Option<Piece>> {
    //     if move_obj.get_piece().is_none() {
    //         return Err(ChuiError::InvalidMove(
    //             "No piece to apply move.".to_string(),
    //         ));
    //     }

    //     let pieces = self.position.get_pieces(move_obj.get_piece().unwrap());

    //     // println!("Pieces: {:?}", pieces);

    //     let mut pieces_can_move = Vec::<Piece>::new();

    //     for piece in pieces.iter() {
    //         if piece.get_move_coords(self, None).iter().any(|&coord| {
    //             coord.get_file() == move_obj.to_coord.get_file()
    //                 && coord.get_rank() == move_obj.to_coord.get_rank()
    //         }) {
    //             pieces_can_move.push(*piece);
    //         }
    //     }

    //     // println!("Pieces can move: {:?}", pieces_can_move);

    //     let (file, rank) = move_obj.to_coord.to_u8_index();

    //     if pieces_can_move.is_empty() {
    //         Err(ChuiError::InvalidMove(format!(
    //             "No {} can move to target square {}",
    //             move_obj.get_piece().unwrap(),
    //             move_obj.to_coord
    //         )))
    //     } else if pieces_can_move.len() == 1 {
    //         let piece = pieces_can_move.get(0).unwrap();
    //         self.get_position_mut().replace_piece(*piece, move_obj)
    //     } else {
    //         Err(ChuiError::InvalidMove(format!(
    //             "Ambiguous move. More than one piece can move to target square {}{}",
    //             file, rank
    //         )))
    //     }
    // }

    //
    // Getters.
    //

    /// Get a reference to the position.
    pub const fn get_position(&self) -> BitPosition {
        self.position
    }

    /// Get a mutable reference to the position.
    pub fn get_position_mut(&mut self) -> &mut BitPosition {
        &mut self.position
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
        self.true_en_passant_target_square = coord;
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

    //
    // Utilities.
    //

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
        Board::new(Variant::StandardChess)
    }
}
