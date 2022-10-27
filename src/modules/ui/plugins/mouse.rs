//! Assets plugin

use bevy::prelude::*;
//use bevy::render::camera::RenderTarget;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use super::{GameState, UiState, MainCamera, get_world_coords, SPRITE_WIDTH};


#[derive(Component)]
struct MouseCursor;

fn init_cursor(
    mut commands: Commands
) {
    let mut rng = SmallRng::seed_from_u64(0 as u64);
    let mut color = Color::from(rng.gen::<[f32; 3]>());
    color.set_a(0.65);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SPRITE_WIDTH, SPRITE_WIDTH)),
                color,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MouseCursor);
}

fn draw_cursor(
    mut mouse_query: Query<&mut Transform, With<MouseCursor>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Res<Windows>,
    ui_state: Res<UiState>
) {
    let mut transform = mouse_query.single_mut();
    let coords = get_world_coords(camera_query, windows);
    let scale = ui_state.square_pixels / SPRITE_WIDTH; // 0.28125 by default

    transform.translation = Vec3::new(coords[0], coords[1], 1.);
    transform.scale = Vec3::new(scale, scale, 0.);
}

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Next).with_system(init_cursor))
            .add_system_set(SystemSet::on_update(GameState::Next).with_system(draw_cursor));
    }
}
