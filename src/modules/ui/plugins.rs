//! Plugins module

pub mod camera;
pub use camera::{CameraControllerPlugin, MainCamera};

pub mod mouse;
pub use mouse::MousePlugin;

pub mod board;
pub use board::BoardPlugin;

pub mod ui_state;
pub use ui_state::{
    UiState, UiStatePlugin, Square,
    update_square_pixels, compute_coords,
    START_X_COORD, START_Y_COORD
};

pub mod assets;
pub use assets::{SpriteCollection, AssetsPlugin, SPRITE_WIDTH};

pub mod egui_panels;
pub use egui_panels::EguiPanelsPlugin;

pub mod game_state;
pub use game_state::{GameState, GameStatePlugin};

pub mod window_descriptor;
pub use window_descriptor::WindowDescriptorPlugin;

pub mod debug;
pub use debug::{Fps, debug_panel, get_mouse_coords, get_world_coords};
