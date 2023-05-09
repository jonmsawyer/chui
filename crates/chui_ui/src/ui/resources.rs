//! Main resources module

use std::fmt;
use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

/// Chui game engine.
pub use chui_core;

/// Resource to engage the core Engine
#[derive(Resource, Debug)]
pub struct Engine(pub chui_core::Engine);

impl Deref for Engine {
    type Target = chui_core::Engine;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Engine {
    fn deref_mut(&mut self) -> &mut chui_core::Engine {
        &mut self.0
    }
}

impl Default for Engine {
    fn default() -> Engine {
        Engine(chui_core::Engine::default())
    }
}

/// Formats the position for white.
impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.white_to_string())
    }
}

/// Resource to keep track of the state of the User Interface.
#[derive(Default, Clone, Resource)]
pub struct UiResource {
    /// Is the window open? (Needed anymore?)
    pub is_window_open: bool,

    /// UI Scale Factor. Understood by `bevy_egui`.
    pub ui_scale_factor: f32,

    /// The status of the last action taken. Displayed in the bottom
    /// panel in the UI.
    pub status: String,

    /// The value of the width of the primary window, in pixels.
    pub window_width: f32,

    /// The value of the height of the primary window, in pixels.
    pub window_height: f32,

    /// The width of the Information Panel, in pixels.
    pub info_panel_width: f32,

    /// The width of of the Annotation Panel, in pixels.
    pub annotation_panel_width: f32,

    /// The computed value of the square pixels of each asset (Square and Piece).
    pub square_pixels: f32,

    /// The margin of the chessboard used to calculate `square_pixels`, in pixels.
    pub board_margin: f32,

    /// The scale factor (percentage) of the pieces drawn on the chessboard. (Needed?)
    pub piece_scale_factor: f32,

    /// True if we render the board according to White's perspective. False if we
    /// render the board according to Black's position.
    pub draw_for_white: bool,

    /// True if we render debug information in the Information panel.
    pub debug_window: bool,

    /// True if we render the mouse chessboard cursor.
    pub show_mouse_cursor: bool,

    /// A vec containing the screen coordinates of the mouse cursor.
    pub mouse_cursor_screen_coords: Vec2,

    /// A vec containing the world coordinates of the mouse cursor.
    pub mouse_cursor_world_coords: Vec2,

    /// A vec containing the world coordinates of a mouse click.
    pub mouse_click_board_coords: Vec2,

    /// A tuple containing the Algebraic coordinates of a mouse click.
    pub mouse_click_algebraic_coords: (char, usize),

    /// A vec containing the world coordinates of the From Square mouse click.
    pub mouse_click_from_square: Vec2,

    /// True if the From Square was mouse clicked.
    pub mouse_click_from_square_clicked: bool,

    /// A vec containing the world coordinates of the To Square mouse click.
    pub mouse_click_to_square: Vec2,

    /// True if the To Square was mouse clicked.
    pub mouse_click_to_square_clicked: bool,

    /// A String representing the move in the currently selected Parser's notation.
    pub move_representation: String,

    /// Camera last position.
    pub camera_last_position: Vec3,

    /// Show board coordinates?
    pub show_coords: bool,
}

/// Resource for calculating our Frames Per Second
#[derive(Debug, Default, Clone, Copy)]
pub struct FpsResource<const N: usize> {
    /// Current average FPS.
    pub average: f32,

    /// Sum of per-frame time deltas.
    pub sum: f32,

    /// Current measurements count since last recalculation.
    pub count: usize,
}

impl<const N: usize> FpsResource<N> {
    /// Add a new time delta measurement.
    pub fn add(&mut self, delta: f32) {
        self.sum += delta;
        self.count = (self.count + 1) % N;

        if self.count == 0 {
            // Average delta would be sum/len, but we want average FPS which is the reciprocal
            self.average = N as f32 / self.sum;
            self.sum = 0.;
        }
    }
}
