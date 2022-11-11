//! Debug plugin.

use bevy::prelude::*;
use bevy_egui::egui::Ui;

use super::super::resources::{FpsResource, UiResource};

/// Render the egui debug panel.
pub fn debug_panel(
    mut ui_state: ResMut<UiResource>,
    mut fps: Local<FpsResource<25>>,
    time: Res<Time>,
    ui: &mut Ui,
) {
    if !ui_state.debug_window {
        return;
    }
    ui.heading("Debug");

    fps.add(time.delta_seconds());

    if fps.average > f32::EPSILON {
        ui.label(format!("FPS: {:.2}", fps.average));
    }
    ui.label(format!(
        "Mouse Cursor Screen Coords: {}, {}",
        ui_state.mouse_cursor_screen_coords[0] as i32,
        ui_state.mouse_cursor_screen_coords[1] as i32
    ));
    ui.label(format!(
        "Mouse Cursor World Coords: {}, {}",
        ui_state.mouse_cursor_world_coords[0] as i32, ui_state.mouse_cursor_world_coords[1] as i32
    ));
    ui.label(format!(
        "Mouse Click Board Coords: {}, {}",
        ui_state.mouse_click_board_coords[0] as i32, ui_state.mouse_click_board_coords[1] as i32
    ));
    ui.label(format!(
        "Mouse Click Algebraic Coords: {}{}",
        ui_state.mouse_click_algebraic_coords.0, ui_state.mouse_click_algebraic_coords.1
    ));
    ui.label(format!(
        "Mouse Click From Square: {}, {}",
        ui_state.mouse_click_from_square[0], ui_state.mouse_click_from_square[1]
    ));
    ui.label(format!(
        "Mouse Click From Square Clicked: {}",
        ui_state.mouse_click_from_square_clicked
    ));
    ui.label(format!(
        "Mouse Click To Square: {}, {}",
        ui_state.mouse_click_to_square[0], ui_state.mouse_click_to_square[1]
    ));
    ui.label(format!(
        "Mouse Click To Square Clicked: {}",
        ui_state.mouse_click_to_square_clicked
    ));
    ui.label(format!(
        "Move Represenation: {}",
        ui_state.move_representation
    ));
    ui.label(format!(
        "Camera Last Position: {}",
        ui_state.camera_last_position.truncate()
    ));
    if ui_state.show_mouse_cursor {
        ui.vertical_centered_justified(|options_ui| {
            options_ui.toggle_value(&mut ui_state.show_mouse_cursor, "Hide Mouse Cursor");
        });
    } else {
        ui.vertical_centered_justified(|options_ui| {
            options_ui.toggle_value(&mut ui_state.show_mouse_cursor, "Show Mouse Cursor");
        });
    }
    ui.separator();
}

// fn _debug_window(
//     mut egui_context: ResMut<EguiContext>,
//     windows: Res<Windows>,
//     mut ui_state: ResMut<UiState>,
//     mut fps: Local<Fps<25>>,
//     time: Res<Time>,
//     query: Query<(&Camera, &GlobalTransform), With<MainCamera>>
// ) {
//     if ui_state.debug_window {
//         let window = match windows.get_primary() {
//             Some(win) => win,
//             None => return
//         };
//         let cursor = get_mouse_coords(window);
//         let coords = get_world_coords(query, windows);

//         fps.add(time.delta_seconds());

//         if fps.average > f32::EPSILON {
//             egui::Window::new("Debug")
//                 .show(egui_context.ctx_mut(), |ui| {
//                     ui.label(format!("FPS: {:.2}", fps.average));
//                     ui.label(format!("Mouse Screen Coords: {}, {}", cursor[0] as i32, cursor[1] as i32));
//                     ui.label(format!("Mouse World Coords: {}, {}", coords[0] as i32, coords[1] as i32));
//                     ui.vertical_centered_justified(|options_ui| {
//                         options_ui.toggle_value(&mut ui_state.show_mouse_cursor, "Show Mouse Cursor");
//                     });
//             });
//         }
//     }
// }

// pub struct DebugPlugin;

// impl Plugin for DebugPlugin {
//     fn build(&self, _app: &mut App) {
//         // app.add_system_set(
//         //     SystemSet::on_update(GameState::Next)
//         //         .with_system(debug_window)
//         //     );
//     }
// }
