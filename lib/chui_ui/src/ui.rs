//! The User Interface

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
//use bevy_editor_pls::prelude::*; // Wait til this is in crates.io
//use bevy_inspector_egui::WorldInspectorPlugin;

pub mod components;
pub mod constants;
pub mod events;
pub mod plugins;
pub mod resources;
pub mod states;
pub mod utils;

/// The main struct for the User Interface. Defines one function called `run()`,
/// which runs the Bevy engine and User Interface.
#[derive(Debug, Copy, Clone)]
pub struct Ui;

impl Ui {
    /// The main function to run the User Interface.
    pub fn run() {
        App::new()
            .add_plugins(plugins::WindowDescriptorPlugin) // Must be first
            //.add_plugins(WorldInspectorPlugin::new()) // bevy_inspector_egui plugin
            .add_plugins(EguiPlugin) // Default Egui plugins
            //.add_plugins(EditorPlugin) // Wait til this is in crates.io
            // Chui's resources
            .init_resource::<resources::Game>()
            // Chui's plugins
            .add_plugins((
                plugins::CameraControllerPlugin,
                plugins::GameStatePlugin,
                plugins::UiStatePlugin,
                plugins::AssetsPlugin,
                plugins::MousePlugin,
                plugins::EguiPanelsPlugin,
                plugins::BoardPlugin,
                plugins::PiecesPlugin
            ))
            // Chui's custom events
            .add_event::<events::ResizeBoardEvent>()
            // Run it
            .run();
    }
}
