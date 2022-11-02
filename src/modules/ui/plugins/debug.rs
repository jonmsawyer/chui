//! Debug plugin.

use bevy::prelude::*;
use bevy_egui::egui::Ui;

use super::super::components::MainCamera;
use super::super::resources::{UiResource, FpsResource};
use super::super::utils::{get_mouse_coords, get_world_coords};


pub fn debug_panel(
    mut ui_state: ResMut<UiResource>,
    windows: Res<Windows>,
    mut fps: Local<FpsResource<25>>,
    time: Res<Time>,
    ui: &mut Ui,
    query: Query<(&Camera, &GlobalTransform), With<MainCamera>>
) {
    if ui_state.debug_window {
        let window = match windows.get_primary() {
            Some(win) => win,
            None => return
        };
        let cursor = get_mouse_coords(window);
        let coords = get_world_coords(query, windows);

        fps.add(time.delta_seconds());

        if fps.average > f32::EPSILON {
            ui.heading("Debug");
            ui.label(format!("FPS: {:.2}", fps.average));
            ui.label(format!("Mouse Screen Coords: {}, {}", cursor[0] as i32, cursor[1] as i32));
            ui.label(format!("Mouse World Coords: {}, {}", coords[0] as i32, coords[1] as i32));
            ui.vertical_centered_justified(|options_ui| {
                options_ui.toggle_value(&mut ui_state.show_mouse_cursor, "Show Mouse Cursor");
            });
            ui.separator();
        }
    }
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
