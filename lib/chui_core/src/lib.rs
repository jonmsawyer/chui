//! Chui Core

#![warn(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![allow(clippy::mod_module_files)]
#![allow(clippy::wildcard_imports)]

pub use chui_error::{ChuiError, ChuiResult};

mod board;
mod chess_move;
mod command;
mod condition;
pub mod constants;
mod coordinate;
mod fen;
mod game;
mod move_generator;
pub mod parser;
mod piece;
mod player;
mod position;
mod traits;
mod variant;

/// Chui Core Prelude
pub mod prelude {
    use super::*;
    pub use crate::{ChuiError, ChuiResult};
    pub use board::Board;
    pub use chess_move::{Move, MoveType};
    pub use command::{Command, CommandContext, CommandKind};
    pub use condition::{DrawCondition, WinCondition};
    pub use constants::*;
    pub use coordinate::{Coord, NonMaxU8};
    pub use fen::Fen;
    pub use game::Game;
    pub use move_generator::MoveGenerator;
    pub use parser::ParserEngine;
    pub use piece::{Color, Piece, PieceKind};
    pub use player::Player;
    pub use position::{Array2D, EasyPosition};
    pub use traits::{Coordinate, Parser, Position};
    pub use variant::Variant;
}

use prelude::*;
