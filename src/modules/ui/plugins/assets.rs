//! Assets plugin

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::Piece;
use super::{GameState, UiState, Square};
use crate::modules::ui::events::ResizeBoardEvent;


const START_X_COORD: f32 = -4.0; // The left four squares of the chessboard, in world coordinates
const START_Y_COORD: f32 = 4.0; // The top four squares of the chessboard, in world coordinates
const SPRITE_WIDTH: f32 = 256.0; // The size of the sprite in x*y dimentions (square)

#[derive(AssetCollection, Clone)]
pub struct SpriteCollection {
    /// Light and dark squares, with chess pieces are defined in a texture atlas (aka sprite sheet).
    ///
    /// Consts cannot be used in attribute macros, so we have to hardcode tile size into here
    #[asset(texture_atlas(tile_size_x = 256., tile_size_y = 256., columns = 14, rows = 1))]
    #[asset(path = "default_board.png")]
    pub tiles: Handle<TextureAtlas>,
    // /// Atlas for our character sprites
    // #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 6, rows = 10))]
    // #[asset(path = "characters.png")]
    // pub characters: Handle<TextureAtlas>,
    // /// Texture atlas for our cursor/selection sprites
    // #[asset(texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 5, rows = 1))]
    // #[asset(path = "cursor.png")]
    // pub cursors: Handle<TextureAtlas>,
    // /// The background image
    // #[asset(path = "nasa-mars.png")]
    // pub background: Handle<Image>,
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
            println!("square.index = {}", square.index);
            transform.translation = Vec3::new(x + offset, y - offset, 0.);
            transform.scale = Vec3::new(scale, scale, 0.);

            x += ui_state.square_pixels;

            if (square.index + 1) % 8 == 0 { // 8 squares per row
                row += 1.0_f32;
                x = start_x;
                y = start_y - (row * ui_state.square_pixels);
            }
        });

        set.p1().iter_mut().for_each(|(piece, mut transform)| {
            let (x, y) = piece.get_coords();

            // I don't know why the next two lines work, but they do, after much deduction.
            let x: f32 = start_x - start_x * x as f32 / 4.0;
            let y: f32 = -(start_y - start_y * y as f32 / 4.0) + (start_y / 4.0);

            transform.translation = Vec3::new(x + offset, y - offset, 0.5);
            transform.scale = Vec3::new(scale*ui_state.piece_scale_factor, scale*ui_state.piece_scale_factor, 0.);
        });
    }
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::Next)
                    .with_collection::<SpriteCollection>()
            )
            .add_system_set(
                SystemSet::on_update(GameState::Next)
                    .with_system(resize_board)
            );
    }
}
