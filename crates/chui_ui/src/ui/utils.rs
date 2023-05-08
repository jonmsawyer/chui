//! Utils module.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::MainCamera;
use super::constants::{FILES, RANKS, SPRITE_WIDTH, START_X_COORD, START_Y_COORD};
use super::resources::UiResource;

/// Transform the from square cursor to the indicated mouse coordinates.
pub fn transform_from_square(
    ui_state: &mut UiResource,
    mut transform: &mut Transform,
    visibility: &mut Visibility,
) {
    let (scale, _, _) = compute_coords(ui_state.square_pixels);
    let (x, y) = if ui_state.draw_for_white {
        (
            (ui_state.mouse_click_from_square[0] - START_Y_COORD) * ui_state.square_pixels,
            (ui_state.mouse_click_from_square[1] - START_Y_COORD) * ui_state.square_pixels,
        )
    } else {
        (
            (7_f32 - ui_state.mouse_click_from_square[0] - START_Y_COORD) * ui_state.square_pixels,
            (7_f32 - ui_state.mouse_click_from_square[1] - START_Y_COORD) * ui_state.square_pixels,
        )
    };
    transform.translation = Vec3::new(x, y, 0.15);
    transform.scale = Vec3::new(scale, scale, 0.);
    *visibility = Visibility::Inherited;
}

/// Transform the to square cursor to to the indicated mouse coordinates.
pub fn transform_to_square(
    ui_state: &mut UiResource,
    mut transform: &mut Transform,
    visibility: &mut Visibility,
) {
    let (scale, _, _) = compute_coords(ui_state.square_pixels);
    let (x, y) = if ui_state.draw_for_white {
        (
            (ui_state.mouse_click_to_square[0] - START_Y_COORD) * ui_state.square_pixels,
            (ui_state.mouse_click_to_square[1] - START_Y_COORD) * ui_state.square_pixels,
        )
    } else {
        (
            (7_f32 - ui_state.mouse_click_to_square[0] - START_Y_COORD) * ui_state.square_pixels,
            (7_f32 - ui_state.mouse_click_to_square[1] - START_Y_COORD) * ui_state.square_pixels,
        )
    };
    transform.translation = Vec3::new(x, y, 0.15);
    transform.scale = Vec3::new(scale, scale, 0.);
    *visibility = Visibility::Inherited;
}

/// Hide the From Square and To Square cursors.
pub fn hide_from_and_to_square(from_visibility: &mut Visibility, to_visibility: &mut Visibility) {
    *from_visibility = Visibility::Hidden;
    *to_visibility = Visibility::Hidden;
}

/// Compute the world coords from the chessboard coordinates (zero-indexed).
pub fn compute_world_coords(coord: (usize, usize), ui_state_square_pixels: f32) -> Vec2 {
    let x = coord.0 as f32;
    let y = coord.1 as f32;
    let world_coords_x = (x - START_Y_COORD) * ui_state_square_pixels;
    let world_coords_y = (y - START_Y_COORD) * ui_state_square_pixels;
    Vec2::new(world_coords_x, world_coords_y)
}

/// Compute the chessboard coordinates (zero-indexed) from mouse click coordinates.
pub fn compute_board_coords(
    mut ui_state: &mut UiResource,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) -> bool {
    let mouse_world_coords = get_world_coords(camera_query, windows);
    let x = (mouse_world_coords[0] / ui_state.square_pixels).floor() + START_Y_COORD;
    let y = (mouse_world_coords[1] / ui_state.square_pixels).floor() + START_Y_COORD;
    let min: f32 = 0.;
    let max: f32 = 7.;

    if x < min || x > max || y < min || y > max {
        return false;
    }

    if ui_state.draw_for_white {
        ui_state.mouse_click_board_coords = Vec2::new(x, y);
        ui_state.mouse_click_algebraic_coords = (FILES[x as usize], RANKS[y as usize]);
    } else {
        ui_state.mouse_click_board_coords = Vec2::new(max - x, max - y);
        ui_state.mouse_click_algebraic_coords = (FILES[7 - x as usize], RANKS[7 - y as usize]);
    }

    true
}

/// Compute the scale, start x, and start y coordinates based on the UI's square pixels.
pub fn compute_coords(square_pixels: f32) -> (f32, f32, f32) {
    let scale = square_pixels / SPRITE_WIDTH; // 0.28125 by default
    let start_x = START_X_COORD * square_pixels; // -288.0 by default
    let start_y = START_Y_COORD * square_pixels - square_pixels; // 216.0 by default

    (scale, start_x, start_y)
}

/// Given the state of the UI, update the square pixels calculation. When the user increases
/// the UI scale factor, this function is called, or upon window resize.
pub fn update_square_pixels(mut ui_state: ResMut<UiResource>) -> ResMut<UiResource> {
    let x_square_pixels = (ui_state.window_width
        - ui_state.board_margin
        - (ui_state.info_panel_width * ui_state.ui_scale_factor)
        - (ui_state.annotation_panel_width * ui_state.ui_scale_factor))
        / 8.0; // 8 columns

    let y_square_pixels = (
        ui_state.window_height -
        ui_state.board_margin -
        (25.0 * ui_state.ui_scale_factor) - // 25.0 pixels for menu bar
        (25.0 * ui_state.ui_scale_factor)
        // 25.0 pixels for status bar
    ) / 8.0; // 8 rows

    if x_square_pixels <= y_square_pixels {
        ui_state.square_pixels = x_square_pixels;
    } else {
        ui_state.square_pixels = y_square_pixels;
    }

    // println!("square_pixels = {}", ui_state.square_pixels);

    ui_state
}

/// Get the screen coordinates of the mouse cursor.
pub fn get_mouse_coords(window: &Window) -> Vec2 {
    window.cursor_position().map_or(Vec2::ZERO, |cursor| cursor)
}

/// Get the world coordinates of the mouse cursor.
pub fn get_world_coords(
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) -> Vec2 {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_query.single();

    let window = match window_query.get_single() {
        Ok(window) => window,
        _ => return Vec2::ZERO,
    };

    window.cursor_position().map_or(Vec2::ZERO, |screen_pos| {
        // get the size of the window
        let window_size = Vec2::new(window.width(), window.height());

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        world_pos
    })
}
