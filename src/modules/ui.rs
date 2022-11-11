//! The User Interface

use bevy::{prelude::*, render::settings::{Backends, WgpuSettings}};
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
            .add_plugin(plugins::WindowDescriptorPlugin) // Must be first
            .insert_resource(WgpuSettings {
                // NOTE: This allows GL support in wgpu, which only has "best-effort" support
                // NOTE: Additionally, wgpu only supports GL on Windows via ANGLE, which may not be available
                backends: Some(Backends::PRIMARY | Backends::GL),
                ..default()
            })
            .add_plugins(DefaultPlugins) // Default Bevy plugins
            .add_plugin(EguiPlugin) // Default Egui plugins
            //.add_plugin(EditorPlugin) // Wait til this is in crates.io
            //.add_plugin(WorldInspectorPlugin::new()) // bevy_inspector_egui plugin
            // Chui's resources
            .init_resource::<resources::Engine>()
            // Chui's custom events
            .add_event::<events::ResizeBoardEvent>()
            // Chui's plugins
            .add_plugin(plugins::GameStatePlugin)
            .add_plugin(plugins::UiStatePlugin)
            .add_plugin(plugins::CameraControllerPlugin)
            .add_plugin(plugins::AssetsPlugin)
            .add_plugin(plugins::MousePlugin)
            .add_plugin(plugins::EguiPanelsPlugin)
            .add_plugin(plugins::BoardPlugin).add_plugin(plugins::PiecesPlugin)
            // Run it
            .run();
    }
}
