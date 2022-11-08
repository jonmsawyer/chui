//! Components module

use bevy::prelude::*;

/// Component to attach to the rendering of the pieces.
pub use crate::Piece;

/// Component to attach to the camera we're controlling.
///
/// Doing this allows us to easily query for it, while also allowing for cameras to
/// exist that we aren't controlling. This is also extensible, as we can add
/// configurable options on a per-camera basis in the future.
#[derive(Component, Default)]
pub struct MainCamera;

/// Component to attach to the mouse cursor.
#[derive(Component)]
pub struct MouseCursor;

/// Component to mark the "from square" mouse cursor.
#[derive(Component)]
pub struct FromSquareCursor;

/// Component to mark the "to square" mouse cursor.
#[derive(Component)]
pub struct ToSquareCursor;

/// Component to attach to each square on the chessboard.
#[derive(Component)]
pub struct Square {
    pub index: usize,
}
