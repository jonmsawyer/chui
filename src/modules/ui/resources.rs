//! Main resources module

/// Chui game engine
pub use crate::Engine;

/// Resource to keep track of the state of the User Interface.
#[derive(Default, Clone)]
pub struct UiResource {
    pub is_window_open: bool,
    pub ui_scale_factor: f32,
    pub status: String,
    pub window_width: f32,
    pub window_height: f32,
    pub info_panel_width: f32,
    pub annotation_panel_width: f32,
    pub square_pixels: f32,
    pub board_margin: f32,
    pub piece_scale_factor: f32,
    pub draw_for_white: bool,
    pub debug_window: bool,
    pub show_mouse_cursor: bool,
}

/// Resource for calculating our Frames Per Second
#[derive(Debug, Default, Clone, Copy)]
pub struct FpsResource<const N: usize> {
    /// Current average FPS
    pub average: f32,
    /// Sum of per-frame time deltas
    pub sum: f32,
    /// Current measurements count since last recalculation
    pub count: usize,
}

impl<const N: usize> FpsResource<N> {
    /// Add a new time delta measurement
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

