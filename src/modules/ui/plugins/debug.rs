//! Assets plugin

use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy_egui::{egui, EguiContext};

use super::{GameState, UiState, MainCamera};


/// Container for calculating our FPS
#[derive(Debug, Default, Clone, Copy)]
struct Fps<const N: usize> {
    /// Current average FPS
    average: f32,
    /// Sum of per-frame time deltas
    sum: f32,
    /// Current measurements count since last recalculation
    count: usize,
}

impl<const N: usize> Fps<N> {
    /// Add a new time delta measurement
    fn add(&mut self, delta: f32) {
        self.sum += delta;
        self.count = (self.count + 1) % N;

        if self.count == 0 {
            // Average delta would be sum/len, but we want average FPS which is the reciprocal
            self.average = N as f32 / self.sum;

            self.sum = 0.;
        }
    }
}

pub fn get_mouse_coords(window: &Window) -> Vec2 {
    match window.cursor_position() {
        Some(cursor) => cursor,
        None => Vec2::ZERO,
    }
}

pub fn get_world_coords(
    query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Res<Windows>,
) -> Vec2 {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = query.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        match windows.get(id) {
            Some(win) => win,
            None => return Vec2::ZERO,
        };
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        world_pos
    } else {
        Vec2::ZERO
    }
}

fn debug_window(
    mut egui_context: ResMut<EguiContext>,
    windows: Res<Windows>,
    mut ui_state: ResMut<UiState>,
    mut fps: Local<Fps<25>>,
    time: Res<Time>,
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
            egui::Window::new("Debug")
                .show(egui_context.ctx_mut(), |ui| {
                    ui.label(format!("FPS: {:.2}", fps.average));
                    ui.label(format!("Mouse Screen Coords: {}, {}", cursor[0] as i32, cursor[1] as i32));
                    ui.label(format!("Mouse World Coords: {}, {}", coords[0] as i32, coords[1] as i32));
                    ui.vertical_centered_justified(|options_ui| {
                        options_ui.toggle_value(&mut ui_state.show_mouse_cursor, "Show Mouse Cursor");
                    });
                });
        }
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Next)
                .with_system(debug_window)
            );
    }
}
