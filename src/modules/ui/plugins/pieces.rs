//! Maintain the state of the User Interface

use bevy::prelude::*;

use crate::{Engine, Piece};
use crate::modules::ui::events::ResizeBoardEvent;
use super::{GameState, SpriteCollection, UiState, compute_coords};


#[derive(Component)]
pub struct Square {
    pub index: usize
}

fn init_pieces(
    my_assets: Res<SpriteCollection>,
    mut commands: Commands,
    ui_state: Res<UiState>,
    engine: Res<Engine>
) {
    let (offset, scale, start_x, start_y) = compute_coords(ui_state.square_pixels);
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

fn resize_pieces(
    ui_state: Res<UiState>,
    mut resize_event: EventReader<ResizeBoardEvent>,
    mut query: Query<(&Piece, &mut Transform)>
    //engine: Res<Engine>
) {
    for _ in resize_event.iter() {
        let (offset, scale, start_x, start_y) = compute_coords(ui_state.square_pixels);

        match ui_state.draw_for_white {
            true => {
                query.for_each_mut(|(piece, mut transform)| {
                    let (x, y) = piece.get_coords();

                    // I don't know why the next two lines work, but they do, after much deduction.
                    let x: f32 = start_x - start_x * x as f32 / 4.0;
                    let y: f32 = -(start_y - start_y * y as f32 / 4.0) + start_y / 4.0;

                    transform.translation = Vec3::new(x + offset, y - offset, 0.5);
                    transform.scale = Vec3::new(scale*ui_state.piece_scale_factor, scale*ui_state.piece_scale_factor, 0.);
                });
            },

            false => {
                query.for_each_mut(|(piece, mut transform)| {
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
pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Next).with_system(init_pieces))
            .add_system_set(SystemSet::on_update(GameState::Next).with_system(resize_pieces));
    }
}
