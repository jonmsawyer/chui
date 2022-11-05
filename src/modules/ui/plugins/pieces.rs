//! Maintain the state of the User Interface

use bevy::prelude::*;
use bevy::sprite::Anchor;

use super::super::components::Piece;
use super::super::constants::{START_X_COORD, START_Y_COORD};
use super::super::events::ResizeBoardEvent;
use super::super::resources::{Engine, UiResource};
use super::super::states::GameState;
use super::super::utils::compute_coords;
use super::SpriteCollection;

fn init_pieces(
    my_assets: Res<SpriteCollection>,
    mut commands: Commands,
    ui_state: Res<UiResource>,
    engine: Res<Engine>,
) {
    let (scale, start_x, start_y) = compute_coords(ui_state.square_pixels);
    let (mut x, mut y, mut row) = (start_x, start_y, 0.);

    engine
        .board
        .get_board()
        .iter()
        .enumerate()
        .for_each(|(x_idx, rank)| {
            rank.iter().enumerate().for_each(|(y_idx, piece)| {
                let idx = (x_idx * 8) + y_idx;
                if let Some(piece) = piece {
                    commands
                        .spawn_bundle(SpriteSheetBundle {
                            transform: Transform {
                                translation: Vec3::new(x, y, 0.5),
                                ..Default::default()
                            }
                            .with_scale(Vec3::new(
                                scale * ui_state.piece_scale_factor,
                                scale * ui_state.piece_scale_factor,
                                0.5,
                            )),
                            //sprite: TextureAtlasSprite::new(piece.get_sprite_index()),
                            sprite: TextureAtlasSprite {
                                index: piece.get_sprite_index(),
                                anchor: Anchor::BottomLeft,
                                ..Default::default()
                            },
                            texture_atlas: my_assets.tiles.clone(),
                            ..Default::default()
                        })
                        .insert(*piece);
                }

                x += ui_state.square_pixels;

                if (idx + 1) % 8 == 0 {
                    // 8 squares per row
                    row += 1.0_f32;
                    x = start_x;
                    y = start_y - (row * ui_state.square_pixels);
                }
            });
        });
}

fn resize_pieces(
    ui_state: Res<UiResource>,
    mut resize_event: EventReader<ResizeBoardEvent>,
    mut query: Query<(&Piece, &mut Transform)>,
) {
    for _ in resize_event.iter() {
        let (scale, _, _) = compute_coords(ui_state.square_pixels);

        match ui_state.draw_for_white {
            true => {
                query.for_each_mut(|(piece, mut transform)| {
                    let (x, y) = piece.get_coords();
                    let x = (x as f32 + START_X_COORD) * ui_state.square_pixels;
                    let y = (y as f32 - START_Y_COORD) * ui_state.square_pixels;

                    transform.translation = Vec3::new(x, y, 0.5);
                    transform.scale = Vec3::new(
                        scale * ui_state.piece_scale_factor,
                        scale * ui_state.piece_scale_factor,
                        0.,
                    );
                });
            }

            false => {
                query.for_each_mut(|(piece, mut transform)| {
                    let (x, y) = piece.get_coords();
                    let x = ((7 - x) as f32 + START_X_COORD) * ui_state.square_pixels;
                    let y = ((7 - y) as f32 - START_Y_COORD) * ui_state.square_pixels;

                    transform.translation = Vec3::new(x, y, 0.5);
                    transform.scale = Vec3::new(
                        scale * ui_state.piece_scale_factor,
                        scale * ui_state.piece_scale_factor,
                        0.,
                    );
                });
            }
        }
    }
}

/// Our UI State plugin
pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Next).with_system(init_pieces))
            .add_system_set(SystemSet::on_update(GameState::Next).with_system(resize_pieces));
    }
}
