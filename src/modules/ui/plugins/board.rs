//! Maintain the state of the User Interface

use bevy::prelude::*;
use bevy::sprite::Anchor;
//use bevy::text::{Text, TextStyle};
use bevy::ui::entity::TextBundle;
use bevy::ui::{AlignSelf, Style};

use crate::modules::ui::constants::START_Y_COORD;

use super::super::components::{BoardBackground, BoardCoordinate, Square};
use super::super::constants::{
    BOARD_BACKGROUND_SPRITE_WIDTH, FILES, INFO_PANEL_WIDTH, RANKS, START_X_COORD,
};
use super::super::events::ResizeBoardEvent;
use super::super::resources::UiResource;
use super::super::states::GameState;
use super::super::utils::compute_coords;
use super::SpriteCollection;

/// ECS System. Run once. Initialize the chessboard.
fn init_board_background(
    my_assets: Res<SpriteCollection>,
    mut commands: Commands,
    ui_state: Res<UiResource>,
) {
    let x = START_X_COORD * ui_state.square_pixels - 25_f32;
    let y = x;
    let scale = (ui_state.square_pixels * 8_f32 + 50_f32) / BOARD_BACKGROUND_SPRITE_WIDTH;

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(x, y, 0.),
                ..Default::default()
            }
            .with_scale(Vec3::new(scale, scale, 0.)),
            // sprite: TextureAtlasSprite::new(color_id),
            sprite: Sprite {
                //custom_size: Some(Vec2::new(custom_size, custom_size)),
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            texture: my_assets.board_background.clone(),
            ..Default::default()
        })
        .insert(BoardBackground);
}

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
                    translation: Vec3::new(x, y, 0.1),
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

/// Initialize the board coordinates.
#[allow(clippy::needless_range_loop)]
fn init_coordinates(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_state: Res<UiResource>,
) {
    let font = asset_server.load("arial.ttf");
    for file_idx in 0..8 {
        let x = (file_idx as f32 * ui_state.square_pixels)
            + INFO_PANEL_WIDTH
            + ui_state.square_pixels
            + (ui_state.square_pixels / START_Y_COORD);
        // let x = 600_f32; //(file_idx as f32 * ui_state.square_pixels) + INFO_PANEL_WIDTH;
        // let y = 55_f32; //last_position[1] + (4_f32 * ui_state.square_pixels) - 25_f32;
        //let y = last_position[1] + (-4_f32 * ui_state.square_pixels);
        let y = ui_state.window_height / 2_f32
            - START_Y_COORD * ui_state.square_pixels
            - (ui_state.board_margin / START_Y_COORD)
            + 6_f32;
        // let (scale, start_x, start_y) = compute_coords(ui_state.square_pixels);
        commands
            .spawn_bundle(
                // Create a TextBundle that has a Text with a single section.
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    // file_idx.to_string(),
                    FILES[file_idx],
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                )
                // Set the alignment of the Text
                .with_text_alignment(TextAlignment::TOP_CENTER)
                // Set the style of the TextBundle itself.
                .with_style(Style {
                    align_self: AlignSelf::FlexStart,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        bottom: Val::Px(y),
                        left: Val::Px(x),
                        ..Default::default()
                    },
                    ..default()
                }),
            )
            .insert(BoardCoordinate {
                file_index: file_idx as isize,
                rank_index: -1_isize,
            });
    }
    for rank_idx in 0..8 {
        let x = INFO_PANEL_WIDTH + ui_state.square_pixels / 2_f32 + 4_f32;
        // let x = 600_f32; //(file_idx as f32 * ui_state.square_pixels) + INFO_PANEL_WIDTH;
        // let y = 55_f32; //last_position[1] + (4_f32 * ui_state.square_pixels) - 25_f32;
        //let y = last_position[1] + (-4_f32 * ui_state.square_pixels);
        let y = (rank_idx as f32 * ui_state.square_pixels) + ui_state.window_height / 2_f32
            - START_Y_COORD * ui_state.square_pixels
            - (ui_state.board_margin / START_Y_COORD)
            + 16_f32
            + ui_state.square_pixels / 2_f32;
        // let (scale, start_x, start_y) = compute_coords(ui_state.square_pixels);
        commands
            .spawn_bundle(
                // Create a TextBundle that has a Text with a single section.
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    // file_idx.to_string(),
                    RANKS[rank_idx].to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                )
                // Set the alignment of the Text
                .with_text_alignment(TextAlignment::TOP_CENTER)
                // Set the style of the TextBundle itself.
                .with_style(Style {
                    align_self: AlignSelf::FlexStart,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        bottom: Val::Px(y),
                        left: Val::Px(x),
                        ..Default::default()
                    },
                    ..default()
                }),
            )
            .insert(BoardCoordinate {
                file_index: -1_isize,
                rank_index: rank_idx as isize,
            });
    }
}

/// Update the board coordinates.
fn update_coordinates(mut query: Query<(&mut Style, &BoardCoordinate)>, ui_state: Res<UiResource>) {
    let last_position = ui_state.camera_last_position.truncate();
    for (mut style, board_coordinate) in query.iter_mut() {
        let mut x: f32;
        let mut y: f32;
        if ui_state.draw_for_white {
            x = if last_position == Vec2::ZERO {
                board_coordinate.file_index as f32 * ui_state.square_pixels
                    + INFO_PANEL_WIDTH
                    + ui_state.square_pixels
                    + ui_state.square_pixels / START_Y_COORD
            } else {
                -last_position[0]
                    + board_coordinate.file_index as f32 * ui_state.square_pixels
                    + INFO_PANEL_WIDTH
                    + ui_state.square_pixels
                    + ui_state.square_pixels / START_Y_COORD
                    - 4_f32
            };
            y = if last_position == Vec2::ZERO {
                board_coordinate.rank_index as f32 * ui_state.square_pixels
                    + ui_state.window_height / 2_f32
                    - START_Y_COORD * ui_state.square_pixels
                    - (ui_state.board_margin / START_Y_COORD)
                    + 16_f32
                    + ui_state.square_pixels / 2_f32
            } else {
                -last_position[1]
                    + board_coordinate.rank_index as f32 * ui_state.square_pixels
                    + ui_state.window_height / 2_f32
                    - START_Y_COORD * ui_state.square_pixels
                    - (ui_state.board_margin / START_Y_COORD)
                    + 16_f32
                    + ui_state.square_pixels / 2_f32
            };
            if board_coordinate.file_index == -1_isize {
                x += 24_f32;
            }
            if board_coordinate.rank_index == -1_isize {
                y += 24_f32;
            }
        } else {
            x = if last_position == Vec2::ZERO {
                (7 - board_coordinate.file_index) as f32 * ui_state.square_pixels
                    + INFO_PANEL_WIDTH
                    + ui_state.square_pixels
                    + ui_state.square_pixels / START_Y_COORD
            } else {
                -last_position[0]
                    + (7 - board_coordinate.file_index) as f32 * ui_state.square_pixels
                    + INFO_PANEL_WIDTH
                    + ui_state.square_pixels
                    + ui_state.square_pixels / START_Y_COORD
                    - 4_f32
            };
            y = if last_position == Vec2::ZERO {
                (7 - board_coordinate.rank_index) as f32 * ui_state.square_pixels
                    + ui_state.window_height / 2_f32
                    - START_Y_COORD * ui_state.square_pixels
                    - (ui_state.board_margin / START_Y_COORD)
                    + 16_f32
                    + ui_state.square_pixels / 2_f32
            } else {
                -last_position[1]
                    + (7 - board_coordinate.rank_index) as f32 * ui_state.square_pixels
                    + ui_state.window_height / 2_f32
                    - START_Y_COORD * ui_state.square_pixels
                    - (ui_state.board_margin / START_Y_COORD)
                    + 16_f32
                    + ui_state.square_pixels / 2_f32
            };
            if board_coordinate.file_index == -1_isize {
                x -= 24_f32;
            }
            if board_coordinate.rank_index == -1_isize {
                y -= 24_f32;
            }
        }
        style.position = UiRect {
            bottom: Val::Px(y),
            left: Val::Px(x),
            ..Default::default()
        };
    }
}

/// ECS System. Run on each frame. Resize the board.
fn resize_board_background(
    ui_state: Res<UiResource>,
    mut resize_event: EventReader<ResizeBoardEvent>,
    mut query: Query<&mut Transform, With<BoardBackground>>,
) {
    for _ in resize_event.iter() {
        let x = START_X_COORD * ui_state.square_pixels - 25_f32;
        let y = x;
        let scale = (ui_state.square_pixels * 8_f32 + 50_f32) / BOARD_BACKGROUND_SPRITE_WIDTH;

        query.for_each_mut(|mut transform| {
            transform.translation = Vec3::new(x, y, 0.);
            transform.scale = Vec3::new(scale, scale, 0.);
        });
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
            transform.translation = Vec3::new(x, y, 0.1);
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
        app.add_system_set(
            SystemSet::on_enter(GameState::Next)
                .with_system(init_board_background)
                .with_system(init_board)
                .with_system(init_coordinates),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Next)
                .with_system(resize_board_background)
                .with_system(resize_board)
                .with_system(update_coordinates),
        );
    }
}
