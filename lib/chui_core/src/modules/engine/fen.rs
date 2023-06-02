//! FEN module.

use super::super::{Color, Engine, Fen};

/// Get the FEN to move character.
pub fn get_fen_to_move(engine: &Engine) -> String {
    match engine.to_move {
        Color::White => "w".to_string(),
        Color::Black => "b".to_string(),
    }
}

/// Get the FEN for castle characters.
pub fn get_fen_castle(engine: &Engine) -> String {
    let mut castle = String::new();

    if engine.white_can_castle_kingside {
        castle = format!("{}{}", castle, "K");
    }

    if engine.white_can_castle_queenside {
        castle = format!("{}{}", castle, "Q");
    }

    if engine.black_can_castle_kingside {
        castle = format!("{}{}", castle, "k");
    }

    if engine.black_can_castle_queenside {
        castle = format!("{}{}", castle, "q");
    }

    castle
}

/// Get the FEN en passant square.
pub fn get_fen_en_passant(engine: &Engine) -> String {
    if let Some(coord) = engine.enpassant_target_square {
        coord.to_string()
    } else {
        "-".to_string()
    }
}

/// Get the X-FEN en passant square.
pub fn get_x_fen_en_passant(engine: &Engine) -> String {
    get_fen_en_passant(engine)
}

/// Get the FEN half-move clock.
pub fn get_fen_half_move_clock(engine: &Engine) -> String {
    engine.half_move_clock.to_string()
}

/// Get the FEN full-move counter.
pub fn get_fen_full_move_counter(engine: &Engine) -> String {
    engine.move_counter.to_string()
}

/// Display the FEN layout of the board.
pub fn get_fen(engine: &Engine) -> String {
    format!(
        "FEN: {}\nX-FEN: {}\nShredder-FEN: {}",
        Fen::get_fen(engine),
        Fen::get_x_fen(engine),
        Fen::get_shredder_fen(engine),
    )
}
