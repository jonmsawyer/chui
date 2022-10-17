//! Window Descriptor plugin

use bevy::{prelude::*, window::{PresentMode, MonitorSelection, WindowResized}};

use super::UiState;
use crate::modules::ui::events::ResizeBoardEvent;
use super::update_square_pixels;


const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct WindowDescriptorPlugin;


fn resize_notificator(
    mut resize_event: EventReader<WindowResized>,
    mut ui_state: ResMut<UiState>,
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
