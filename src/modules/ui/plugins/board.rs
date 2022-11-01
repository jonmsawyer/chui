//! Maintain the state of the User Interface

use bevy::prelude::*;

use crate::{Engine, Piece};
use crate::modules::ui::events::ResizeBoardEvent;
use super::{GameState, SpriteCollection, UiState, compute_coords};
use super::super::constants::{SPRITE_WIDTH, START_X_COORD, START_Y_COORD};


#[derive(Component)]
pub struct Square {
    pub index: usize
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

fn resize_board(
    ui_state: Res<UiState>,
    mut resize_event: EventReader<ResizeBoardEvent>,
    mut set: ParamSet<(Query<(&Square, &mut Transform)>, Query<(&Piece, &mut Transform)>)>
    //engine: Res<Engine>
) {
    for _ in resize_event.iter() {
        let offset = ui_state.square_pixels / 2.0_f32; // by half because textures are centered
        let scale = ui_state.square_pixels / SPRITE_WIDTH; // 0.28125 by default
        let start_x = START_X_COORD * SPRITE_WIDTH * scale; // -288.0 by default
        let start_y = START_Y_COORD * SPRITE_WIDTH * scale; // 288.0 by default

        let mut x = start_x;
        let mut y = start_y;
        let mut row: f32 = 0.;

        set.p0().iter_mut().for_each(|(square, mut transform)| {
            transform.translation = Vec3::new(x + offset, y - offset, 0.);
            transform.scale = Vec3::new(scale, scale, 0.);

            x += ui_state.square_pixels;

            if (square.index + 1) % 8 == 0 { // 8 squares per row
                row += 1.0_f32;
                x = start_x;
                y = start_y - (row * ui_state.square_pixels);
            }
        });

        match ui_state.draw_for_white {
            true => {
                set.p1().iter_mut().for_each(|(piece, mut transform)| {
                    let (x, y) = piece.get_coords();

                    // I don't know why the next two lines work, but they do, after much deduction.
                    let x: f32 = start_x - start_x * x as f32 / 4.0;
                    let y: f32 = -(start_y - start_y * y as f32 / 4.0) + start_y / 4.0;

                    transform.translation = Vec3::new(x + offset, y - offset, 0.5);
                    transform.scale = Vec3::new(scale*ui_state.piece_scale_factor, scale*ui_state.piece_scale_factor, 0.);
                });
            },

            false => {
                set.p1().iter_mut().for_each(|(piece, mut transform)| {
                    let (x, y) = piece.get_coords();

                    // I don't know why the next two lines work, but they do, after much deduction.
                    let x: f32 = -(start_x - start_x * x as f32 / 4.0) + start_x / 4.0;
                    let y: f32 = start_y - start_y * y as f32 / 4.0;

                    transform.translation = Vec3::new(x + offset, y - offset, 0.5);
                    transform.scale = Vec3::new(scale*ui_state.piece_scale_factor, scale*ui_state.piece_scale_factor, 0.);
                });
            }
        }
    }
}

/// Our UI State plugin
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Next).with_system(init_board))
            .add_system_set(SystemSet::on_update(GameState::Next).with_system(resize_board));
    }
}
