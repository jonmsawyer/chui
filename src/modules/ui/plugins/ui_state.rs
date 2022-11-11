//! Maintain the state of the User Interface

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiSettings};

use super::super::constants::{ANNOTATION_PANEL_WIDTH, INFO_PANEL_WIDTH};
use super::super::events::ResizeBoardEvent;
use super::super::resources::UiResource;
use super::super::utils::update_square_pixels;

/// ECS System. Run once. Configure the state of the User Interface.
fn configure_state(mut ui_state: ResMut<UiResource>) {
    ui_state.is_window_open = false;
    ui_state.ui_scale_factor = 1.0;
    ui_state.status = String::from("Chui Loaded");
    ui_state.window_width = 1280.0; // Bevy window default
    ui_state.window_height = 720.0; // Bevy window default
    ui_state.info_panel_width = INFO_PANEL_WIDTH;
    ui_state.annotation_panel_width = ANNOTATION_PANEL_WIDTH;
    ui_state.square_pixels = 72.0;
    ui_state.board_margin = 104.0;
    ui_state.piece_scale_factor = 1.0;
    ui_state.draw_for_white = true;
    ui_state.debug_window = true;
    ui_state.show_mouse_cursor = true;
    ui_state.mouse_cursor_screen_coords = Vec2::ZERO;
    ui_state.mouse_cursor_world_coords = Vec2::ZERO;
    ui_state.mouse_click_board_coords = Vec2::ZERO;
    ui_state.mouse_click_algebraic_coords = ('-', 9);
    ui_state.mouse_click_from_square = Vec2::ZERO;
    ui_state.mouse_click_from_square_clicked = false;
    ui_state.mouse_click_to_square = Vec2::ZERO;
    ui_state.mouse_click_to_square_clicked = false;
    ui_state.move_representation = "No move selected.".to_string();
    ui_state.camera_last_position = Vec3::new(0., 0., 0.);
    ui_state.show_coords = true;
}

/// ECS System. Run once. Configure the User Interface visuals.
fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    // Default is Dark Mode
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: (5.0).into(), // 5 points radius for window borders
        ..Default::default()
    });
}

/// ECS System. Run on each frame. Given the proper keyboard input, update
/// the UI Scale Factor as understood by egui.
fn scale_factor(
    keyboard_input: Res<Input<KeyCode>>,
    mut egui_settings: ResMut<EguiSettings>,
    mut ui_state: ResMut<UiResource>,
    mut resize_board_event: EventWriter<ResizeBoardEvent>,
) {
    if keyboard_input.pressed(KeyCode::LControl)
        && (keyboard_input.just_pressed(KeyCode::Equals)
            || keyboard_input.just_pressed(KeyCode::NumpadAdd))
    {
        ui_state.ui_scale_factor += 0.1;
        if ui_state.ui_scale_factor > 2.0 {
            ui_state.ui_scale_factor = 2.0;
        }
        ui_state = update_square_pixels(ui_state);
        // Notify that the board should be resized
        resize_board_event.send_default();
    }

    if keyboard_input.pressed(KeyCode::LControl)
        && (keyboard_input.just_pressed(KeyCode::Minus)
            || keyboard_input.just_pressed(KeyCode::NumpadSubtract))
    {
        ui_state.ui_scale_factor -= 0.1;
        if ui_state.ui_scale_factor < -0.2 {
            ui_state.ui_scale_factor = -0.2;
        }
        ui_state = update_square_pixels(ui_state);
        // Notify that the board should be resized
        resize_board_event.send_default();
    }

    if keyboard_input.pressed(KeyCode::LControl)
        && (keyboard_input.just_pressed(KeyCode::Key0)
            || keyboard_input.just_pressed(KeyCode::Numpad0))
    {
        ui_state.ui_scale_factor = 1.0;
        ui_state = update_square_pixels(ui_state);
        // Notify that the board should be resized
        resize_board_event.send_default();
    }

    egui_settings.scale_factor = ui_state.ui_scale_factor as f64;
}

/// Our UI State plugin
pub struct UiStatePlugin;

impl Plugin for UiStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiResource>()
            .insert_resource(ClearColor(Color::BLACK))
            .add_startup_system(configure_state)
            .add_startup_system(configure_visuals)
            .add_system(scale_factor);
    }
}
