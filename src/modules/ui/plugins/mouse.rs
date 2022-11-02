//! Assets plugin

use bevy::prelude::*;
//use bevy::render::camera::RenderTarget;

use bevy::sprite::Anchor;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use super::super::constants::{SPRITE_WIDTH, START_X_COORD, START_Y_COORD};
use super::super::components::{MainCamera, MouseCursor};
use super::super::resources::UiResource;
use super::super::states::GameState;
use super::super::utils::{compute_coords, get_world_coords};


fn init_cursor(
    mut commands: Commands
) {
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

fn update_cursor(
    mut mouse_query: Query<(&mut Visibility, &mut Transform), With<MouseCursor>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Res<Windows>,
    ui_state: Res<UiResource>
) {
    let coords = get_world_coords(camera_query, windows);
    let (mut visibility, mut transform) = mouse_query.single_mut();
    let (_, scale, _, _) = compute_coords(ui_state.square_pixels);
    let x = (coords[0] / ui_state.square_pixels).floor() * ui_state.square_pixels;
    let y = (coords[1] / ui_state.square_pixels).floor() * ui_state.square_pixels;
    let min = START_X_COORD * ui_state.square_pixels;
    let max = START_Y_COORD * ui_state.square_pixels;

    if x < min || x >= max || y < min || y >= max || (coords[0] == 0. && coords[1] == 0.) {
        visibility.is_visible = false;
        return;
    }

    transform.translation = Vec3::new(x, y, 0.2);
    transform.scale = Vec3::new(scale, scale, 0.);
    visibility.is_visible = ui_state.show_mouse_cursor;
}

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Next).with_system(init_cursor))
            .add_system_set(SystemSet::on_update(GameState::Next).with_system(update_cursor));
    }
}
