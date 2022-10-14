//! Window Descriptor plugin

use bevy::{prelude::*, window::{PresentMode, MonitorSelection, WindowResized}};

use super::UiState;


const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct WindowDescriptorPlugin;

fn resize_notificator(
    resize_event: Res<Events<WindowResized>>,
    mut ui_state: ResMut<UiState>
) {
    let mut reader = resize_event.get_reader();
    for window in reader.iter(&resize_event) {
        println!("width = {} height = {}", window.width, window.height);
        ui_state.window_width = window.width;
        ui_state.window_height = window.height;
        ui_state.square_pixels = (
            ui_state.window_width -
            ui_state.board_margin -
            ui_state.info_panel_width -
            ui_state.annotation_panel_width
        ) / 8.0_f32;
        println!("square_pixels = {}", ui_state.square_pixels);
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
