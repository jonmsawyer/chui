//! Utils module.

use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

use super::constants::{SPRITE_WIDTH, START_X_COORD, START_Y_COORD};
use super::resources::UiResource;
use super::components::MainCamera;


pub fn compute_coords(square_pixels: f32) -> (f32, f32, f32, f32) {
    let offset = square_pixels / 2.; // by half because textures are centered
    let scale = square_pixels / SPRITE_WIDTH; // 0.28125 by default
    let start_x = START_X_COORD * SPRITE_WIDTH * scale; // -288.0 by default
    let start_y = START_Y_COORD * SPRITE_WIDTH * scale; // 288.0 by default

    (offset, scale, start_x, start_y)
}

pub fn update_square_pixels(mut ui_state: ResMut<UiResource>) -> ResMut<UiResource> {
    let x_square_pixels = (
        ui_state.window_width -
        ui_state.board_margin -
        (ui_state.info_panel_width * ui_state.ui_scale_factor) -
        (ui_state.annotation_panel_width * ui_state.ui_scale_factor)
    ) / 8.0; // 8 columns

    let y_square_pixels = (
        ui_state.window_height -
        ui_state.board_margin -
        (25.0 * ui_state.ui_scale_factor) - // 25.0 pixels for menu bar
        (25.0 * ui_state.ui_scale_factor)   // 25.0 pixels for status bar
    ) / 8.0; // 8 rows

    if x_square_pixels <= y_square_pixels {
        ui_state.square_pixels = x_square_pixels;
    }
    else {
        ui_state.square_pixels = y_square_pixels;
    }

    // println!("square_pixels = {}", ui_state.square_pixels);

    ui_state
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
    let window = if let RenderTarget::Window(id) = camera.target {
        match windows.get(id) {
            Some(win) => win,
            None => return Vec2::ZERO,
        }
    } else {
        windows.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = window.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

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
