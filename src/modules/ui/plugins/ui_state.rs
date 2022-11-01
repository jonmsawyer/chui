//! Maintain the state of the User Interface

use bevy::prelude::*;
use bevy_egui::{egui, EguiSettings, EguiContext};

use super::SpriteCollection;
use super::egui_panels::INFO_PANEL_WIDTH;
use super::egui_panels::ANNOTATION_PANEL_WIDTH;
use super::GameState;
use crate::Engine;
use crate::modules::ui::events::ResizeBoardEvent;


pub const START_X_COORD: f32 = -4.0; // The left four squares of the chessboard, in world coordinates
pub const START_Y_COORD: f32 = 4.0; // The top four squares of the chessboard, in world coordinates
const SPRITE_WIDTH: f32 = 256.0; // The size of the sprite in x*y dimentions (square)

#[derive(Default, Clone)]
pub struct UiState {
    pub is_window_open: bool,
    pub ui_scale_factor: f32,
    pub status: String,
    pub window_width: f32,
    pub window_height: f32,
    pub info_panel_width: f32,
    pub annotation_panel_width: f32,
    pub square_pixels: f32,
    pub board_margin: f32,
    pub piece_scale_factor: f32,
    pub draw_for_white: bool,
    pub debug_window: bool,
    pub show_mouse_cursor: bool,
}

#[derive(Component)]
pub struct Square {
    pub index: usize
}

fn configure_ui_state(mut ui_state: ResMut<UiState>) {
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

pub fn update_square_pixels(mut ui_state: ResMut<UiState>) -> ResMut<UiState> {
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

fn update_ui_scale_factor(
    keyboard_input: Res<Input<KeyCode>>,
    mut egui_settings: ResMut<EguiSettings>,
    mut ui_state: ResMut<UiState>,
    mut resize_board_event: EventWriter<ResizeBoardEvent>
) {
    if keyboard_input.pressed(KeyCode::LControl) &&
       keyboard_input.just_pressed(KeyCode::Equals)
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
       keyboard_input.just_pressed(KeyCode::Minus)
    {
        ui_state.ui_scale_factor -= 0.1;
        if ui_state.ui_scale_factor < 1.0 {
            ui_state.ui_scale_factor = 1.0;
        }
        ui_state = update_square_pixels(ui_state);
        // Notify that the board should be resized
        resize_board_event.send_default();
    }

    egui_settings.scale_factor = ui_state.ui_scale_factor as f64;
}

fn configure_ui_visuals(mut egui_ctx: ResMut<EguiContext>) {
    // Default is Dark Mode
    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: (5.0).into(), // 5 points radius for window borders
        ..Default::default()
    });
}

pub fn compute_coords(square_pixels: f32) -> (f32, f32, f32, f32) {
    let offset = square_pixels / 2.; // by half because textures are centered
    let scale = square_pixels / SPRITE_WIDTH; // 0.28125 by default
    let start_x = START_X_COORD * SPRITE_WIDTH * scale; // -288.0 by default
    let start_y = START_Y_COORD * SPRITE_WIDTH * scale; // 288.0 by default

    (offset, scale, start_x, start_y)
}

fn init_board(
    my_assets: Res<SpriteCollection>,
    mut commands: Commands,
    ui_state: Res<UiState>,
    engine: Res<Engine>
) {
    let (offset, scale, start_x, start_y) = compute_coords(ui_state.square_pixels);

    let (mut x, mut y, mut row) = (start_x, start_y, 0.);

    for idx in 0..64 { // 64 squares in a chessboard
        // color_id will be 0 for a light square and 1 for a dark square.
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
            }).insert(Square { index: idx });

        x += ui_state.square_pixels;

        if (idx + 1) % 8 == 0 { // 8 squares per row
            row += 1.0_f32;
            x = start_x;
            y = start_y - (row * ui_state.square_pixels);
        }
    }

    let (mut x, mut y, mut row) = (start_x, start_y, 0.);

    engine.board.get_board().iter().enumerate().for_each(|(x_idx, rank)| {
        rank.iter().enumerate().for_each(|(y_idx, piece)| {
            let idx = (x_idx * 8) + y_idx;
            if let Some(piece) = piece {
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        transform: Transform {
                            translation: Vec3::new(x + offset, y - offset, 0.5),
                            ..Default::default()
                        }.with_scale(Vec3::new(scale*ui_state.piece_scale_factor, scale*ui_state.piece_scale_factor, 0.5)),
                        sprite: TextureAtlasSprite::new(piece.get_sprite_index()),
                        texture_atlas: my_assets.tiles.clone(),
                        ..Default::default()
                    }).insert(*piece);
            }

            x += ui_state.square_pixels;

            if (idx + 1) % 8 == 0 { // 8 squares per row
                row += 1.0_f32;
                x = start_x;
                y = start_y - (row * ui_state.square_pixels);
            }
        });
    });
}

/// Our UI State plugin
pub struct UiStatePlugin;

impl Plugin for UiStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .insert_resource(ClearColor(Color::BLACK))
            .add_startup_system(configure_ui_state)
            .add_startup_system(configure_ui_visuals)
            .add_system(update_ui_scale_factor)
            .add_system_set(SystemSet::on_enter(GameState::Next).with_system(init_board));
    }
}
