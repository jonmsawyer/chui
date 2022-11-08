//! Assets plugin

use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::sprite::Anchor;
//use bevy::render::camera::RenderTarget;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use super::super::components::{FromSquareCursor, MainCamera, MouseCursor, ToSquareCursor};
use super::super::constants::{SPRITE_WIDTH, START_X_COORD, START_Y_COORD};
use super::super::resources::UiResource;
use super::super::states::GameState;
use super::super::utils::{
    compute_board_coords, compute_coords, get_mouse_coords, get_world_coords,
    hide_from_and_to_square, transform_from_square, transform_to_square,
};

fn init_mouse_cursor(mut commands: Commands) {
    let mut rng = SmallRng::seed_from_u64(1 as u64);
    let mut color = Color::from(rng.gen::<[f32; 3]>());
    color.set_a(0.65);

    commands
        .spawn_bundle(SpriteBundle {
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
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(MouseCursor);
}

fn init_from_square_cursor(mut commands: Commands) {
    let mut rng = SmallRng::seed_from_u64(2 as u64);
    let mut color = Color::from(rng.gen::<[f32; 3]>());
    color.set_a(0.65);

    commands
        .spawn_bundle(SpriteBundle {
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
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(FromSquareCursor);
}

fn init_to_square_cursor(mut commands: Commands) {
    let mut rng = SmallRng::seed_from_u64(3 as u64);
    let mut color = Color::from(rng.gen::<[f32; 3]>());
    color.set_a(0.65);

    commands
        .spawn_bundle(SpriteBundle {
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
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(ToSquareCursor);
}

fn update_mouse_cursor(
    mut mouse_query: Query<(&mut Visibility, &mut Transform), With<MouseCursor>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Res<Windows>,
    mut ui_state: ResMut<UiResource>,
) {
    let window = match windows.get_primary() {
        Some(win) => win,
        None => return,
    };
    let mouse_coords = get_mouse_coords(window);
    let world_coords = get_world_coords(camera_query, windows);
    let (mut visibility, mut transform) = mouse_query.single_mut();
    let (scale, _, _) = compute_coords(ui_state.square_pixels);
    let x = (world_coords[0] / ui_state.square_pixels).floor() * ui_state.square_pixels;
    let y = (world_coords[1] / ui_state.square_pixels).floor() * ui_state.square_pixels;
    let min = START_X_COORD * ui_state.square_pixels;
    let max = START_Y_COORD * ui_state.square_pixels;

    ui_state.mouse_cursor_screen_coords = mouse_coords;
    ui_state.mouse_cursor_world_coords = world_coords.clone();

    if x < min
        || x >= max
        || y < min
        || y >= max
        || (world_coords[0] == 0. && world_coords[1] == 0.)
    {
        visibility.is_visible = false;
        return;
    }

    transform.translation = Vec3::new(x, y, 0.2);
    transform.scale = Vec3::new(scale, scale, 0.);
    visibility.is_visible = ui_state.show_mouse_cursor;
}

pub fn update_mouse_click(
    mut ui_state: ResMut<UiResource>,
    windows: Res<Windows>,
    mut mouse_input: EventReader<MouseButtonInput>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut from_square_query: Query<(&mut Transform, &mut Visibility), With<FromSquareCursor>>,
    mut to_square_query: Query<
        (&mut Transform, &mut Visibility),
        (With<ToSquareCursor>, Without<FromSquareCursor>),
    >,
) {
    if mouse_input.is_empty() {
        return;
    }

    compute_board_coords(&mut ui_state, camera_query, windows);

    for input in mouse_input.iter() {
        match (input.button, input.state) {
            (MouseButton::Left, ButtonState::Pressed) => {
                let (mut from_transform, mut from_visibility) = from_square_query.single_mut();
                let (mut to_transform, mut to_visibility) = to_square_query.single_mut();

                if !ui_state.mouse_click_from_square_clicked
                    && !ui_state.mouse_click_to_square_clicked
                {
                    ui_state.mouse_click_from_square_clicked = true;
                    ui_state.mouse_click_from_square = ui_state.mouse_click_board_coords.clone();
                    transform_from_square(&mut ui_state, &mut from_transform, &mut from_visibility);
                } else if ui_state.mouse_click_from_square_clicked
                    && !ui_state.mouse_click_to_square_clicked
                {
                    ui_state.mouse_click_to_square_clicked = true;
                    ui_state.mouse_click_to_square = ui_state.mouse_click_board_coords.clone();
                    transform_to_square(&mut ui_state, &mut to_transform, &mut to_visibility);
                } else if ui_state.mouse_click_from_square_clicked
                    && ui_state.mouse_click_to_square_clicked
                {
                    ui_state.mouse_click_from_square_clicked = false;
                    ui_state.mouse_click_from_square = Vec2::ZERO;
                    ui_state.mouse_click_to_square_clicked = false;
                    ui_state.mouse_click_to_square = Vec2::ZERO;
                    hide_from_and_to_square(&mut from_visibility, &mut to_visibility);
                }
            }
            _ => {}
        }

        // match mouse_input.into() {
        //     MouseButton::Left => {
        //         ui_state.mouse_click_coords = mouse_world_coords.clone();
        //         //ui_state.mouse_click_board_coords = board_coords.clone();
        //         compute_board_coords(&mut ui_state);
        //     },
        //     _ => {}
        // }
    }
}

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Next)
                .with_system(init_mouse_cursor)
                .with_system(init_from_square_cursor)
                .with_system(init_to_square_cursor),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Next)
                .with_system(update_mouse_cursor)
                .with_system(update_mouse_click),
        );
    }
}
