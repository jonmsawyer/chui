//! Window Descriptor plugin

use bevy::{prelude::*, window::PresentMode};


const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct WindowDescriptorPlugin;

impl Plugin for WindowDescriptorPlugin {
    fn build(&self, app: &mut App) {
        // Main window, with title.
        //
        // Detect dragging in the menu bar (but not on a menu), and use
        // Window::set_position(Window::position() + drag_delta) or something like that,
        // the function names are similar if I didn't get them exactly right, but they're
        // on the Window object.
        // -Travis Veazey <https://github.com/Kromey>
        app.insert_resource(WindowDescriptor {
            title: format!(r#"Chui: Chess UI v{}"#, VERSION),
            present_mode: PresentMode::AutoVsync,
            decorations: true,
            ..default()
        });
    }
}
