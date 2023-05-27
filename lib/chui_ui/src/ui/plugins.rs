//! Plugins module

pub mod camera;
pub use camera::CameraControllerPlugin;

pub mod mouse;
pub use mouse::MousePlugin;

pub mod board;
pub use board::BoardPlugin;

pub mod pieces;
pub use pieces::PiecesPlugin;

pub mod ui_state;
pub use ui_state::UiStatePlugin;

pub mod assets;
pub use assets::{AssetsPlugin, SpriteCollection};

pub mod egui_panels;
pub use egui_panels::EguiPanelsPlugin;

pub mod game_state;
pub use game_state::GameStatePlugin;

pub mod window_descriptor;
pub use window_descriptor::WindowDescriptorPlugin;

pub mod debug;
pub use debug::debug_panel;
