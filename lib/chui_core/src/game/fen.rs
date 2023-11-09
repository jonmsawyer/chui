//! FEN module.

use super::super::{Color, Fen, Game};

/// Get the FEN to move character.
pub fn get_fen_to_move(game: &Game) -> String {
    match game.to_move {
        Color::White => "w".to_string(),
        Color::Black => "b".to_string(),
    }
}

/// Get the FEN for castle characters.
pub fn get_fen_castle(game: &Game) -> String {
    let mut castle = String::new();

    if game.board.white_can_castle_kingside {
        castle = format!("{}{}", castle, "K");
    }

    if game.board.white_can_castle_queenside {
        castle = format!("{}{}", castle, "Q");
    }

    if game.board.black_can_castle_kingside {
        castle = format!("{}{}", castle, "k");
    }

    if game.board.black_can_castle_queenside {
        castle = format!("{}{}", castle, "q");
    }

    castle
}

/// Get the FEN en passant square.
pub fn get_fen_en_passant(game: &Game) -> String {
    game.board
        .get_en_passant_coord()
        .map_or_else(|| "-".to_string(), |coord| coord.to_string())
}

/// Get the X-FEN en passant square.
pub fn get_x_fen_en_passant(game: &Game) -> String {
    get_fen_en_passant(game)
}

/// Get the FEN half-move clock.
pub fn get_fen_half_move_clock(game: &Game) -> String {
    game.half_move_clock.to_string()
}

/// Get the FEN full-move counter.
pub fn get_fen_full_move_counter(game: &Game) -> String {
    game.move_counter.to_string()
}

/// Display the FEN layout of the board.
pub fn get_fen(game: &Game) -> String {
    format!(
        "FEN: {}\nX-FEN: {}\nShredder-FEN: {}",
        Fen::get_fen(game),
        Fen::get_x_fen(game),
        Fen::get_shredder_fen(game),
    )
}
