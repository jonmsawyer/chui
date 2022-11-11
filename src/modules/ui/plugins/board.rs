//! Maintain the state of the User Interface

use bevy::prelude::*;
use bevy::sprite::Anchor;

use super::super::components::Square;
use super::super::resources::UiResource;
use super::super::states::GameState;
use super::super::utils::compute_coords;
use super::SpriteCollection;
use crate::modules::ui::events::ResizeBoardEvent;

/// ECS System. Run once. Initialize the chessboard.
fn init_board(my_assets: Res<SpriteCollection>, mut commands: Commands, ui_state: Res<UiResource>) {
    let (scale, start_x, start_y) = compute_coords(ui_state.square_pixels);
    let (mut x, mut y, mut row) = (start_x, start_y, 0.);

    for idx in 0..64 {
        // 64 squares in a chessboard
        // color_id will be 0 for a light square and 1 for a dark square.
        let color_id = ((idx / 8) % 2 + idx % 2) % 2; // 8 squares per row

        commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    ..Default::default()
                }
                .with_scale(Vec3::new(scale, scale, 0.)),
                // sprite: TextureAtlasSprite::new(color_id),
                sprite: TextureAtlasSprite {
                    index: color_id,
                    anchor: Anchor::BottomLeft,
                    ..Default::default()
                },
                texture_atlas: my_assets.tiles.clone(),
                ..Default::default()
            })
            .insert(Square { index: idx });

        x += ui_state.square_pixels;

        if (idx + 1) % 8 == 0 {
            // 8 squares per row
            row += 1.0_f32;
            x = start_x;
            y = start_y - (row * ui_state.square_pixels);
        }
    }
}

/// ECS System. Run on each frame. Resize the board.
fn resize_board(
    ui_state: Res<UiResource>,
    mut resize_event: EventReader<ResizeBoardEvent>,
    mut query: Query<(&Square, &mut Transform)>,
) {
    for _ in resize_event.iter() {
        let (scale, start_x, start_y) = compute_coords(ui_state.square_pixels);

        let mut x = start_x;
        let mut y = start_y;
        let mut row: f32 = 0.;

        query.for_each_mut(|(square, mut transform)| {
            transform.translation = Vec3::new(x, y, 0.);
            transform.scale = Vec3::new(scale, scale, 0.);

            x += ui_state.square_pixels;

            if (square.index + 1) % 8 == 0 {
                // 8 squares per row
                row += 1.0_f32;
                x = start_x;
                y = start_y - (row * ui_state.square_pixels);
            }
        });
    }
}

/// Our UI State plugin
pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Next).with_system(init_board))
            .add_system_set(SystemSet::on_update(GameState::Next).with_system(resize_board));
    }
}
