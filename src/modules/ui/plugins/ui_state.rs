//! Maintain the state of the User Interface

use bevy::prelude::*;
use bevy_egui::{egui, EguiSettings, EguiContext};

use super::super::events::ResizeBoardEvent;
use super::super::resources::UiResource;
use super::super::utils::update_square_pixels;
use super::super::constants::{INFO_PANEL_WIDTH, ANNOTATION_PANEL_WIDTH};


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
}

fn scale_factor(
    keyboard_input: Res<Input<KeyCode>>,
    mut egui_settings: ResMut<EguiSettings>,
    mut ui_state: ResMut<UiResource>,
    mut resize_board_event: EventWriter<ResizeBoardEvent>
) {
    if keyboard_input.pressed(KeyCode::LControl) &&
       (keyboard_input.just_pressed(KeyCode::Equals) ||
        keyboard_input.just_pressed(KeyCode::NumpadAdd))
    {
        ui_state.ui_scale_factor += 0.1;
        if ui_state.ui_scale_factor > 2.0 {
            ui_state.ui_scale_factor = 2.0;
        }
        ui_state = update_square_pixels(ui_state);
        // Notify that the board should be resized
        resize_board_event.send_default();
    }

    if keyboard_input.pressed(KeyCode::LControl) &&
       (keyboard_input.just_pressed(KeyCode::Minus) ||
        keyboard_input.just_pressed(KeyCode::NumpadSubtract))
    {
        ui_state.ui_scale_factor -= 0.1;
        if ui_state.ui_scale_factor < -0.2 {
            ui_state.ui_scale_factor = -0.2;
        }
        ui_state = update_square_pixels(ui_state);
        // Notify that the board should be resized
        resize_board_event.send_default();
    }

    if keyboard_input.pressed(KeyCode::LControl) &&
       (keyboard_input.just_pressed(KeyCode::Key0) ||
        keyboard_input.just_pressed(KeyCode::Numpad0))
    {
        ui_state.ui_scale_factor = 1.0;
        ui_state = update_square_pixels(ui_state);
        // Notify that the board should be resized
        resize_board_event.send_default();
    }

    egui_settings.scale_factor = ui_state.ui_scale_factor as f64;
}

fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    // Default is Dark Mode
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: (5.0).into(), // 5 points radius for window borders
        ..Default::default()
    });
}

/// Our UI State plugin
pub struct UiStatePlugin;

impl Plugin for UiStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiResource>()
            .insert_resource(ClearColor(Color::BLACK))
            .add_startup_system(configure_state)
            .add_startup_system(configure_visuals)
            .add_system(scale_factor);
    }
}
