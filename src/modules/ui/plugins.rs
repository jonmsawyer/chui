//! Plugins module

pub mod camera;
pub use camera::{CameraControllerPlugin, MainCamera};

pub mod mouse;
pub use mouse::MousePlugin;

pub mod ui_state;
pub use ui_state::{UiState, UiStatePlugin, Square, update_square_pixels, compute_coords};

pub mod assets;
pub use assets::{SpriteCollection, AssetsPlugin, SPRITE_WIDTH};

pub mod main_ui;
pub use main_ui::MainUiPlugin;

pub mod game_state;
pub use game_state::{GameState, GameStatePlugin};

pub mod window_descriptor;
pub use window_descriptor::WindowDescriptorPlugin;

pub mod debug;
pub use debug::{DebugPlugin, get_mouse_coords, get_world_coords};
