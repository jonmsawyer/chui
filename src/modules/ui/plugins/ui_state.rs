//! Maintain the state of the User Interface

use bevy::prelude::*;
use bevy_egui::{egui, EguiSettings, EguiContext};

#[derive(Default)]
pub struct UiState {
    _label: String,
    _value: f32,
    _egui_texture_handle: Option<egui::TextureHandle>,
    is_window_open: bool,
    scale_factor: f64,
}

fn configure_ui_state(mut ui_state: ResMut<UiState>) {
    ui_state.is_window_open = false;
}

fn update_ui_scale_factor(
    keyboard_input: Res<Input<KeyCode>>,
    mut egui_settings: ResMut<EguiSettings>,
    mut ui_state: ResMut<UiState>,
    //toggle_scale_factor: Local<Option<bool>>,
    //windows: Res<Windows>,
) {
    if keyboard_input.pressed(KeyCode::LControl) &&
       keyboard_input.just_pressed(KeyCode::Equals)
    {
        // println!("LControl + Equals");
        ui_state.scale_factor += 0.1;
    }
    if keyboard_input.pressed(KeyCode::LControl) &&
       keyboard_input.just_pressed(KeyCode::Minus)
    {
        // println!("LControl + Minus");
        ui_state.scale_factor -= 0.1;
    }
    if ui_state.scale_factor < 1.0 {
        // println!("scale_factor < 1.0, setting to 1.0");
        ui_state.scale_factor = 1.0;
    }
    if ui_state.scale_factor > 2.0 {
        // println!("scale_factor > 2.0, setting to 2.0");
        ui_state.scale_factor = 2.0;
    }
    // println!("scale_factor is currently {}", ui_state.scale_factor);
    egui_settings.scale_factor = ui_state.scale_factor;
    // if keyboard_input.just_pressed(KeyCode::Slash) || toggle_scale_factor.is_none() {
    //     *toggle_scale_factor = Some(!toggle_scale_factor.unwrap_or(true));

    //     if let Some(window) = windows.get_primary() {
    //         let scale_factor = if toggle_scale_factor.unwrap() {
    //             1.0
    //         } else {
    //             1.0 / window.scale_factor()
    //         };
    //         egui_settings.scale_factor = scale_factor;
    //     }
    // }
}

fn configure_ui_visuals(mut egui_ctx: ResMut<EguiContext>) {
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
        app.init_resource::<UiState>()
            // Color used to clear the buffer between frames.
            // It's a "background" for unrendered content.
            .insert_resource(ClearColor(Color::BLACK))
            .add_startup_system(configure_ui_state)
            .add_startup_system(configure_ui_visuals)
            .add_system(update_ui_scale_factor);
    }
}
