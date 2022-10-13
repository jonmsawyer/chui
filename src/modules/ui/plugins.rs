//! Plugins module

pub mod camera;
pub use camera::CameraControllerPlugin;

pub mod ui_state;
pub use ui_state::{UiState, UiStatePlugin};

pub mod assets;
pub use assets::{SpriteCollection, AssetsPlugin};

pub mod main_ui;
pub use main_ui::MainUiPlugin;

pub mod game_state;
pub use game_state::{GameState, GameStatePlugin};

pub mod window_descriptor;
pub use window_descriptor::WindowDescriptorPlugin;
