//! Constants module.

/// The left four squares of the chessboard, in world coordinates.
pub const START_X_COORD: f32 = -4.0;

/// The top four squares of the chessboard, in world coordinates.
pub const START_Y_COORD: f32 = 4.0;

/// The size of the sprite in x*y dimentions (square).
pub const SPRITE_WIDTH: f32 = 256.0;

/// The width of the Information egui panel.
pub const INFO_PANEL_WIDTH: f32 = 300.0;

/// The width of the Annotation egui panel.
pub const ANNOTATION_PANEL_WIDTH: f32 = 300.0;

/// Chessboard files
pub const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

/// Chessboard ranks
pub const RANKS: [usize; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
