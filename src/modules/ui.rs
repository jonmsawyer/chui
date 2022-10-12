//! The User Interface

use bevy::{prelude::*, window::PresentMode};
use bevy_egui::EguiPlugin;
//use bevy_editor_pls::prelude::*; // Wait til this is in crates.io
use bevy_inspector_egui::WorldInspectorPlugin;

pub mod plugins;

pub struct Ui;

const VERSION: &str = env!("CARGO_PKG_VERSION");


#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    Next,
}

impl Ui {
    pub fn run() {
        App::new()
            // Main window, with title
            .insert_resource(WindowDescriptor {
                title: format!(r#"Chui: Chess UI v{}"#, VERSION),
                present_mode: PresentMode::AutoVsync,
                // detect dragging in the menu bar (but not on a menu), and use Window::set_position(Window::position() + drag_delta)
                // or something like that, the function names are similar if i didn't get them exactly right, but they're on the Window object
                // -Kromey (https://github.com/kromey)
                decorations: true,
                ..default()
            })

            //Start off with the default loading state (AssetLoading) and then
            // once the AssetLoading is finished, moved onto the Next state.
            .add_state(GameState::AssetLoading)

            // Set multi-sample anti-aliasing (WGPU only supports 1 or 4)
            .insert_resource(Msaa { samples: 4 })

            // Default Bevy
            .add_plugins(DefaultPlugins)

            // Egui Plugins
            .add_plugin(EguiPlugin)

            // bevy_editor_pls Plugin
            //.add_plugin(EditorPlugin) // Wait til this is in crates.io

            // bevy-inspector-egui Plugin
            .add_plugin(WorldInspectorPlugin::new())

            // Chui's plugins
            .add_plugin(plugins::UiStatePlugin)
            .add_plugin(plugins::CameraControllerPlugin)
            .add_plugin(plugins::AssetsPlugin)
            .add_plugin(plugins::MainUiPlugin)
            .run();
    }
}
