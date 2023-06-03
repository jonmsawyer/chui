//! Chui: Chess UI

#![warn(missing_docs)]
#![deny(broken_intra_doc_links)]

pub use chui_error::{ChuiError, ChuiResult};

mod modules;
pub use modules::{
    constants::*,
    coord::{Coord, NonMaxU8},
    parser::{self, ParserEngine},
    piece::{Color, Piece, PieceKind},
    traits, Board, ChessVariant, Command, CommandContext, CommandKind, Engine, Fen, Move,
    MoveGenerator, MoveType, Player,
};
