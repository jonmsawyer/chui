//! Window Descriptor plugin

use bevy::{
    prelude::*,
    window::{MonitorSelection, PresentMode, WindowResized},
};

use super::super::events::ResizeBoardEvent;
use super::super::resources::UiResource;
use super::super::utils::update_square_pixels;

/// Chui's version.
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// ECS System. Run on each frame. Notify Bevy that the window has been resized,
/// then send a `ResizeBoardEvent`.
fn resize_notificator(
    mut resize_event: EventReader<WindowResized>,
    mut ui_state: ResMut<UiResource>,
    mut resize_board_event: EventWriter<ResizeBoardEvent>,
) {
    for window in resize_event.iter() {
        // println!("width = {} height = {}", window.width, window.height);

        ui_state.window_width = window.width;
        ui_state.window_height = window.height;
        ui_state = update_square_pixels(ui_state);

        // Notify that the board should be resized
        resize_board_event.send_default();
    }
}

/// Window Descriptor Bevy plugin.
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
            position: WindowPosition::Centered(MonitorSelection::Current),
            present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .add_system(resize_notificator);
    }
}
