//! Maintain the state of the User Interface

use bevy::prelude::*;
use bevy_egui::{egui, EguiSettings, EguiContext};

use super::SpriteCollection;
use super::main_ui::INFO_PANEL_WIDTH;
use super::main_ui::ANNOTATION_PANEL_WIDTH;
use super::GameState;

const START_X_COORD: f32 = -4.0; // The left four squares of the chessboard, in world coordinates
const START_Y_COORD: f32 = 4.0; // The top four squares of the chessboard, in world coordinates
const SPRITE_WIDTH: f32 = 256.0; // The size of the sprite in x*y dimentions (square)

#[derive(Default)]
pub struct UiState {
    pub is_window_open: bool,
    pub scale_factor: f64,
    pub status: String,
    pub window_width: f32,
    pub window_height: f32,
    pub info_panel_width: f32,
    pub annotation_panel_width: f32,
    pub square_pixels: f32,
    pub board_margin: f32,
}

fn configure_ui_state(mut ui_state: ResMut<UiState>) {
    ui_state.is_window_open = false;
    ui_state.status = String::from("Chui Loaded");
    ui_state.window_width = 1280.0; // Bevy window default
    ui_state.window_height = 720.0; // Bevy window default
    ui_state.info_panel_width = INFO_PANEL_WIDTH;
    ui_state.annotation_panel_width = ANNOTATION_PANEL_WIDTH;
    ui_state.square_pixels = 72.0;
    ui_state.board_margin = 104.0;
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

fn _init_board(
    my_assets: Res<SpriteCollection>,
    mut commands: Commands,
    ui_state: Res<UiState>
) {
    let offset = ui_state.square_pixels / 2.0_f32; // by half because textures are centered
    let scale = ui_state.square_pixels / SPRITE_WIDTH; // 0.28125 by default
    let start_x = START_X_COORD * SPRITE_WIDTH * scale; // -288.0 by default
    let start_y = START_Y_COORD * SPRITE_WIDTH * scale; // 288.0 by default
    let mut x = start_x;
    let mut y = start_y;
    let mut row: f32 = 0.;
    println!(
        "offset = {}, scale = {}, start_x = {}, start_y = {}, x = {}, y = {}",
        offset, scale, start_x, start_y, x, y
    );

    for idx in 0..64 { // 64 squares in a chessboard
        let color_id = ((idx / 8) % 2 + idx % 2) %2; // 8 squares per row

        commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(x + offset, y - offset, 0.),
                    ..Default::default()
                }.with_scale(Vec3::new(scale, scale, 0.)),
                sprite: TextureAtlasSprite::new(color_id),
                texture_atlas: my_assets.tiles.clone(),
                ..Default::default()
            });

        x += ui_state.square_pixels;

        if (idx + 1) % 8 == 0 { // 8 squares per row
            row += 1.0_f32;
            x = start_x;
            y = start_y - (row * ui_state.square_pixels);
        }
    }
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
            // .add_system_set(
            //     SystemSet::on_enter(GameState::Next)
            //         .with_system(init_board)
            // )
            .add_system(update_ui_scale_factor);
    }
}
