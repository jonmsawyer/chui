//! Assets plugin

use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::{Color, *};
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;
//use bevy::render::camera::RenderTarget;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use super::super::components::{FromSquareCursor, MainCamera, MouseCursor, Piece, ToSquareCursor};
use super::super::constants::{SPRITE_WIDTH, START_X_COORD, START_Y_COORD};
use super::super::resources::{Game, UiResource};
use super::super::states::GameState;
use super::super::utils::{
    compute_board_coords, compute_coords, get_mouse_coords, get_world_coords,
    hide_from_and_to_square, transform_from_square, transform_to_square,
};
use crate::ui::utils::compute_world_coords;
use chui_core::prelude::*;

/// ECS System. Run once. Initialize the on-board mouse cursor.
fn init_mouse_cursor(mut commands: Commands) {
    let mut rng = SmallRng::seed_from_u64(1_u64);
    let mut color = Color::from(rng.gen::<[f32; 3]>());
    color.set_a(0.65);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SPRITE_WIDTH, SPRITE_WIDTH)),
                color,
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 1.0),
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(MouseCursor);
}

/// ECS System. Run once. Initialize the From Square on-board cursor.
fn init_from_square_cursor(mut commands: Commands) {
    let mut rng = SmallRng::seed_from_u64(2_u64);
    let mut color = Color::from(rng.gen::<[f32; 3]>());
    color.set_a(0.65);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SPRITE_WIDTH, SPRITE_WIDTH)),
                color,
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 1.0),
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(FromSquareCursor);
}

/// ECS System. Run once. Initialize the To Square on-board cursor.
fn init_to_square_cursor(mut commands: Commands) {
    let mut rng = SmallRng::seed_from_u64(3_u64);
    let mut color = Color::from(rng.gen::<[f32; 3]>());
    color.set_a(0.65);

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SPRITE_WIDTH, SPRITE_WIDTH)),
                color,
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 1.0),
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .insert(ToSquareCursor);
}

/// ECS System. Run on each frame. Update the on-board mouse cursor.
fn update_mouse_cursor(
    mut mouse_query: Query<(&mut Visibility, &mut Transform), With<MouseCursor>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut ui_state: ResMut<UiResource>,
) {
    let window = match window_query.get_single() {
        Ok(win) => win,
        _ => return,
    };
    let mouse_coords = get_mouse_coords(window);
    let world_coords = get_world_coords(camera_query, window_query);
    let (mut visibility, mut transform) = mouse_query.single_mut();
    let (scale, _, _) = compute_coords(ui_state.square_pixels);
    let x = (world_coords[0] / ui_state.square_pixels).floor() * ui_state.square_pixels;
    let y = (world_coords[1] / ui_state.square_pixels).floor() * ui_state.square_pixels;
    let min = START_X_COORD * ui_state.square_pixels;
    let max = START_Y_COORD * ui_state.square_pixels;

    ui_state.mouse_cursor_screen_coords = mouse_coords;
    ui_state.mouse_cursor_world_coords = world_coords;

    if x < min
        || x >= max
        || y < min
        || y >= max
        || (world_coords[0] == 0. && world_coords[1] == 0.)
    {
        *visibility = Visibility::Hidden;
        return;
    }

    transform.translation = Vec3::new(x, y, 0.2);
    transform.scale = Vec3::new(scale, scale, 0.);
    if ui_state.show_mouse_cursor {
        *visibility = Visibility::Inherited;
    } else {
        *visibility = Visibility::Hidden;
    }
}

/// ECS System. Run on each frame. Update the on-board From Square and To Square
/// mouse cursors on each mouse click.
///
/// # Panics
///
/// Panics if a new `Coord` could not be constructed.
///
/// TODO: Mitigate panics.
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
pub fn update_mouse_click(
    mut ui_state: ResMut<UiResource>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut mouse_input: EventReader<MouseButtonInput>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut from_square_query: Query<(&mut Transform, &mut Visibility), With<FromSquareCursor>>,
    mut to_square_query: Query<
        (&mut Transform, &mut Visibility),
        (With<ToSquareCursor>, Without<FromSquareCursor>),
    >,
    mut game: ResMut<Game>,
    mut piece_query: Query<
        (&mut Piece, &mut Transform),
        (Without<FromSquareCursor>, Without<ToSquareCursor>),
    >,
) {
    if mouse_input.is_empty() {
        return;
    }

    compute_board_coords(&mut ui_state, camera_query, windows);

    for input in mouse_input.read() {
        if let (MouseButton::Left, ButtonState::Pressed) = (input.button, input.state) {
            let (mut from_transform, mut from_visibility) = from_square_query.single_mut();
            let (mut to_transform, mut to_visibility) = to_square_query.single_mut();

            if !ui_state.mouse_click_from_square_clicked && !ui_state.mouse_click_to_square_clicked
            {
                ui_state.mouse_click_from_square_clicked = true;
                ui_state.mouse_click_from_square = ui_state.mouse_click_board_coords;
                transform_from_square(&mut ui_state, &mut from_transform, &mut from_visibility);
            } else if ui_state.mouse_click_from_square_clicked
                && !ui_state.mouse_click_to_square_clicked
            {
                // If the "from" square is equal to the "to" square, zero out fields and return.
                if ui_state.mouse_click_from_square == ui_state.mouse_click_board_coords {
                    ui_state.mouse_click_from_square_clicked = false;
                    ui_state.mouse_click_from_square = Vec2::ZERO;
                    ui_state.mouse_click_to_square_clicked = false;
                    ui_state.mouse_click_to_square = Vec2::ZERO;
                    hide_from_and_to_square(&mut from_visibility, &mut to_visibility);
                    return;
                }

                ui_state.mouse_click_to_square_clicked = true;
                ui_state.mouse_click_to_square = ui_state.mouse_click_board_coords;

                transform_to_square(&mut ui_state, &mut to_transform, &mut to_visibility);
                let from_index = (
                    ui_state.mouse_click_from_square[0] as usize,
                    ui_state.mouse_click_from_square[1] as usize,
                );

                let to_index = (
                    ui_state.mouse_click_to_square[0] as usize,
                    ui_state.mouse_click_to_square[1] as usize,
                );

                let from_coord = Coord::try_from(from_index).unwrap();
                let to_coord = Coord::try_from(to_index).unwrap();

                match game
                    .parser
                    .generate_move_from_board_coordinates(&game, from_coord, to_coord)
                {
                    Ok(result) => {
                        ui_state.move_representation = result;
                        let mut chess_move = ChessMove::new(game.to_move);
                        chess_move.from_coord =
                            Coord::try_from((from_coord.get_file(), from_coord.get_rank())).ok();
                        chess_move.to_coord =
                            Coord::try_from((to_coord.get_file(), to_coord.get_rank())).ok();
                        let from_piece = game.board.get_position().get_piece(Some(from_coord));
                        let to_piece = game.board.get_position().get_piece(Some(to_coord));
                        chess_move.from_piece = from_piece;

                        if from_piece.is_none() {
                            return;
                        }

                        let kind = from_piece.unwrap().get_kind();

                        if to_piece.is_none() {
                            chess_move.move_type = match kind {
                                PieceKind::Pawn => Some(MoveType::PawnMove),
                                _ => Some(MoveType::PieceMove),
                            };
                        } else {
                            chess_move.move_type = match kind {
                                PieceKind::Pawn => Some(MoveType::PawnCapture),
                                _ => Some(MoveType::PieceCapture),
                            };
                        }

                        game.set_current_move(Some(chess_move));

                        match game.apply_move() {
                            Ok(_) => (),
                            Err(_) => return,
                        }

                        piece_query.for_each_mut(|(mut piece, mut transform)| {
                            if piece.get_coord() == Coord::try_from(from_index).unwrap() {
                                piece
                                    .set_coord(Coord::new(to_index.0 as u8, to_index.1 as u8).ok());
                                let world_coords = compute_world_coords(
                                    Coord::try_from(to_index).unwrap(),
                                    ui_state.square_pixels,
                                );
                                transform.translation.x = world_coords.x;
                                transform.translation.y = world_coords.y;
                            }
                        });
                    }
                    Err(error) => ui_state.move_representation = format!("{}", error),
                }
            } else if ui_state.mouse_click_from_square_clicked
                && ui_state.mouse_click_to_square_clicked
            {
                ui_state.mouse_click_from_square_clicked = false;
                ui_state.mouse_click_from_square = Vec2::ZERO;
                ui_state.mouse_click_to_square_clicked = false;
                ui_state.mouse_click_to_square = Vec2::ZERO;
                hide_from_and_to_square(&mut from_visibility, &mut to_visibility);
            }
        };

        // match mouse_input.into() {
        //     MouseButton::Left => {
        //         ui_state.mouse_click_Coords = mouse_world_Coords.clone();
        //         //ui_state.mouse_click_board_Coords = board_Coords.clone();
        //         compute_board_Coords(&mut ui_state);
        //     },
        //     _ => {}
        // }
    }
}

/// Mouse Bevy plugin.
#[derive(Debug, Copy, Clone, Hash)]
pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Next),
            (
                init_mouse_cursor,
                init_from_square_cursor,
                init_to_square_cursor,
            ),
        )
        .add_systems(
            Update,
            (update_mouse_cursor, update_mouse_click).run_if(in_state(GameState::Next)),
        );
    }
}
