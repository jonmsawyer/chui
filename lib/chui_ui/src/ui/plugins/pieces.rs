//! Maintain the state of the User Interface

use bevy::prelude::*;
// use bevy::sprite::Anchor;

use super::super::components::Piece;
use super::super::constants::{START_X_COORD, START_Y_COORD};
use super::super::events::ResizeBoardEvent;
use super::super::resources::{Game, UiResource};
use super::super::states::GameState;
use super::super::utils::compute_coords;
use super::SpriteCollection;

/// ECS System. Run once. Initialize all of the pieces on the chessboard.
const fn init_pieces(
    _my_assets: Res<SpriteCollection>,
    mut _commands: Commands,
    _ui_state: Res<UiResource>,
    _engine: Res<Game>,
) {
    // let (scale, _, _) = compute_coords(ui_state.square_pixels);

    // engine
    //     .board
    //     .get_position()
    //     .iter()
    //     .enumerate()
    //     .for_each(|(_, rank)| {
    //         rank.iter().enumerate().for_each(|(_, piece)| {
    //             if let Some(piece) = piece {
    //                 let piece = Piece::new(*piece);
    //                 let coord = piece.get_coord();
    //                 let x = (coord.get_file() as f32 + START_X_COORD) * ui_state.square_pixels;
    //                 let y = (coord.get_rank() as f32 - START_Y_COORD) * ui_state.square_pixels;
    //                 commands
    //                     .spawn(SpriteSheetBundle {
    //                         transform: Transform {
    //                             translation: Vec3::new(x, y, 0.5),
    //                             ..Default::default()
    //                         }
    //                         .with_scale(Vec3::new(
    //                             scale * ui_state.piece_scale_factor,
    //                             scale * ui_state.piece_scale_factor,
    //                             0.5,
    //                         )),
    //                         //sprite: TextureAtlasSprite::new(piece.get_sprite_index()),
    //                         sprite: TextureAtlasSprite {
    //                             index: piece.get_sprite_index(),
    //                             anchor: Anchor::BottomLeft,
    //                             ..Default::default()
    //                         },
    //                         texture_atlas: my_assets.tiles.clone(),
    //                         ..Default::default()
    //                     })
    //                     .insert(piece);
    //             }
    //         });
    //     });
}

/// ECS System. Run on each frame. Resize the pieces on the chessboard given a
/// `ResizeBoardEvent`.
fn resize_pieces(
    ui_state: Res<UiResource>,
    mut resize_event: EventReader<ResizeBoardEvent>,
    mut query: Query<(&Piece, &mut Transform)>,
) {
    for _ in resize_event.read() {
        let (scale, _, _) = compute_coords(ui_state.square_pixels);

        match ui_state.draw_for_white {
            true => {
                query.for_each_mut(|(piece, mut transform)| {
                    let coord = piece.get_coord();
                    let x = (coord.get_file() as f32 + START_X_COORD) * ui_state.square_pixels;
                    let y = (coord.get_rank() as f32 - START_Y_COORD) * ui_state.square_pixels;

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
                    let coord = piece.get_coord();
                    let x =
                        ((7 - coord.get_file()) as f32 + START_X_COORD) * ui_state.square_pixels;
                    let y =
                        ((7 - coord.get_rank()) as f32 - START_Y_COORD) * ui_state.square_pixels;

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
#[derive(Debug, Copy, Clone, Hash)]
pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Next), init_pieces)
            .add_systems(Update, resize_pieces.run_if(in_state(GameState::Next)));
    }
}
