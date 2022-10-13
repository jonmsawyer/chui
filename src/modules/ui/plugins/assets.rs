//! Assets plugin

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::GameState;


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

fn use_assets(
    my_assets: Res<SpriteCollection>,
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    // do something using the asset handles from the resource
    println!("Use my assets!");
    //draw the original image (whole atlas)
    let atlas = texture_atlases
        .get(&my_assets.tiles)
        .expect("Failed to find our atlas");
    commands.spawn_bundle(SpriteBundle {
        texture: atlas.texture.clone(),
        transform: Transform::from_xyz(0., 0., 1.).with_scale(Vec3::new(0.5, 0.5, 1.0)),
        ..Default::default()
    });
    // draw single texture from sprite sheet starting at index 0
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 1.),
                ..Default::default()
            }.with_scale(Vec3::new(0.5, 0.5, 1.)),
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: my_assets.tiles.clone(),
            ..Default::default()
        });
        //.insert(atlas);
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
                    .with_system(use_assets)
            );
    }
}
