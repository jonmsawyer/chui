//! Components module

use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

use chui_core;

/// Component to attach to the rendering of the pieces.
#[derive(Debug, Component, Copy, Clone, Hash)]
pub struct Piece(chui_core::prelude::Piece);

impl Piece {
    /// Return a new [`Piece`].
    pub const fn new(piece: chui_core::prelude::Piece) -> Piece {
        Piece(piece)
    }
}

impl Deref for Piece {
    type Target = chui_core::prelude::Piece;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Piece {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Component to attach to the camera we're controlling.
///
/// Doing this allows us to easily query for it, while also allowing for cameras to
/// exist that we aren't controlling. This is also extensible, as we can add
/// configurable options on a per-camera basis in the future.
#[derive(Debug, Component, Copy, Clone, Hash)]
pub struct MainCamera;

/// Component to attach to the mouse cursor.
#[derive(Debug, Component, Copy, Clone, Hash)]
pub struct MouseCursor;

/// Component to mark the "from square" mouse cursor.
#[derive(Debug, Component, Copy, Clone, Hash)]
pub struct FromSquareCursor;

/// Component to mark the "to square" mouse cursor.
#[derive(Debug, Component, Copy, Clone, Hash)]
pub struct ToSquareCursor;

/// Component to attach to each square on the chessboard.
#[derive(Debug, Component, Copy, Clone, Hash)]
pub struct Square {
    /// The index associated with this Square. 0-63 (one for each square
    /// on the chessboard).
    pub index: usize,
}

/// Component representing the board background.
#[derive(Debug, Component, Copy, Clone, Hash)]
pub struct BoardBackground;

/// Component representing board Coordinates
#[derive(Debug, Component, Copy, Clone, Hash)]
pub struct BoardCoordinate {
    /// File index. -1 to 7.
    pub file_index: isize,

    /// Rank index. -1 to 7.
    pub rank_index: isize,
}
