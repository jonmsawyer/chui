//! Maintain the state of the User Interface

use bevy::prelude::*;

use crate::Engine;
use super::{GameState, SpriteCollection, UiState, compute_coords};
//use crate::modules::ui::events::ResizeBoardEvent;


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

/// Our UI State plugin
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Next).with_system(init_board));
    }
}
