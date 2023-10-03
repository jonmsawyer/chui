//! Chui Core

mod board;
mod chess_move;
mod command;
mod condition;
mod constants;
mod coordinate;
mod fen;
mod game;
mod move_generator;
mod parser;
mod piece;
mod player;
mod position;
mod result;
mod traits;
mod variant;

/// Chui Core Prelude
pub mod prelude {
    use super::*;
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
    pub use position::{
        Array2D, ArrayBitPosition, BitPosition, BitmaskArray, EasyPosition, EnumArray,
        EnumPosition, PieceEnum,
    };
    pub use rand;
    pub use result::{ChuiError, ChuiResult};
    pub use traits::{Coordinate, Parser, Position};
    pub use variant::Variant;
}

use prelude::*;
