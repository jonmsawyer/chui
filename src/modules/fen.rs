//! FEN notation module.
//!
//! FEN stands for Forsyth-Edwards Notation.

use super::Engine;

/// Represents the FEN notation of a chess position.
#[derive(Debug, Copy, Clone)]
pub struct Fen;

impl Fen {
    /// Get the FEN layout of the board.
    pub fn get_fen(engine: &Engine) -> String {
        let mut fen = Fen::get_board_fen(engine);

        // To move.
        fen = format!("{} {}", fen, engine.get_fen_to_move());

        // Castling.
        fen = format!("{} {}", fen, engine.get_fen_castle());

        // En passant target sqaure.
        fen = format!("{} {}", fen, engine.get_fen_en_passant());

        // Half-move clock since last Pawn move or piece capture.
        fen = format!("{} {}", fen, engine.get_fen_half_move_clock());

        // Full-move counter.
        fen = format!("{} {}", fen, engine.get_fen_full_move_counter());

        fen
    }

    /// Get the Shredder-FEN layout of the board.
    pub fn get_shredder_fen(engine: &Engine) -> String {
        let mut fen = Fen::get_board_fen(engine);

        // To move.
        fen = format!("{} {}", fen, engine.get_fen_to_move());

        // Castling.
        fen = format!("{} {}", fen, engine.get_fen_castle());

        // En passant target sqaure.
        fen = format!("{} {}", fen, engine.get_fen_en_passant());

        // Half-move clock since last Pawn move or piece capture.
        fen = format!("{} {}", fen, engine.get_fen_half_move_clock());

        // Full-move counter.
        fen = format!("{} {}", fen, engine.get_fen_full_move_counter());

        fen
    }

    /// Get the X-FEN layout of the board.
    pub fn get_x_fen(engine: &Engine) -> String {
        let mut fen = Fen::get_board_fen(engine);

        // To move.
        fen = format!("{} {}", fen, engine.get_fen_to_move());

        // Castling.
        fen = format!("{} {}", fen, engine.get_fen_castle());

        // En passant target sqaure.
        fen = format!("{} {}", fen, engine.get_x_fen_en_passant());

        // Half-move clock since last Pawn move or piece capture.
        fen = format!("{} {}", fen, engine.get_fen_half_move_clock());

        // Full-move counter.
        fen = format!("{} {}", fen, engine.get_fen_full_move_counter());

        fen
    }

    /// Get FEN layout of the board only without the other
    /// attributes.
    ///
    /// # Panics
    ///
    /// * Panics if `piece` is None after checking that it is Some.
    pub fn get_board_fen(engine: &Engine) -> String {
        let mut fen = String::new();
        let mut empty_squares = 0;

        // Get board layout.
        for rank in engine.board.get_board().iter().rev() {
            for piece in rank.iter() {
                if piece.is_some() {
                    if empty_squares > 0 {
                        fen = format!(
                            "{}{}{}",
                            fen,
                            empty_squares,
                            piece.expect("Piece cannot be None.")
                        );
                        empty_squares = 0;
                    } else {
                        fen = format!("{}{}", fen, piece.expect("Piece cannot be None."));
                    }
                } else {
                    empty_squares += 1;
                }
            }

            // Write out the number of empty squares between
            // pieces on the same rank.
            if empty_squares > 0 {
                fen = format!("{}{}", fen, empty_squares);
            }

            // Separate ranks by '/'.
            fen = format!("{}/", fen);
            empty_squares = 0;
        }

        // Remove trailing '/'.
        fen.pop();

        fen
    }
}
