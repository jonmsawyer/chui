//! Assets plugin

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::{GameState, UiState};


const START_X_COORD: f32 = -4.0; // The left four squares of the chessboard, in world coordinates
const START_Y_COORD: f32 = 4.0; // The top four squares of the chessboard, in world coordinates
const SPRITE_WIDTH: f32 = 256.0; // The size of the sprite in x*y dimentions (square)

#[derive(AssetCollection)]
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

    // Thanks to Travis Veazey <https://github.com/Kromey>
    // fn display_board_colors_by_index() {
    //     for idx in 0..64 {
    //         let color_id = ((idx / 8) % 2 + idx % 2) % 2;
    //         print!("{}  ", color_id);

    //         if (idx + 1) % 8 == 0 {
    //             println!();
    //         }
    //     }
    // }

fn draw_board(
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

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::Next)
                    .with_collection::<SpriteCollection>()
            )
            .add_system_set(
                SystemSet::on_enter(GameState::Next)
                    .with_system(draw_board)
            );
    }
}
