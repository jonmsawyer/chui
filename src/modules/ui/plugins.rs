//! Plugins module

pub mod camera;
pub use camera::CameraControllerPlugin;

pub mod ui_state;
pub use ui_state::UiState;
pub use ui_state::UiStatePlugin;

pub mod assets;
pub use assets::SpriteCollection;
pub use assets::AssetsPlugin;

pub mod main_ui;
pub use main_ui::MainUiPlugin;
