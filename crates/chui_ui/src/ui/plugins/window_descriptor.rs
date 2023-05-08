//! Window Descriptor plugin

use bevy::{
    prelude::*,
    render::{
        settings::{Backends, WgpuSettings},
        RenderPlugin,
    },
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
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: format!(r#"Chui: Chess UI v{}"#, VERSION),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: (1280., 720.).into(),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(RenderPlugin {
                    wgpu_settings: WgpuSettings {
                        // NOTE: This allows GL support in wgpu, which only has "best-effort" support
                        // NOTE: Additionally, wgpu only supports GL on Windows via ANGLE, which may not be available
                        backends: Some(Backends::PRIMARY | Backends::GL),
                        ..default()
                    },
                }),
        )
        .add_system(resize_notificator);
    }
}
