//! Assets plugin

use bevy::prelude::*;
//use bevy::render::camera::RenderTarget;

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use super::{
    GameState, UiState, MainCamera,
    get_world_coords, compute_coords,
    SPRITE_WIDTH
};


#[derive(Component)]
struct MouseCursor;

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

fn draw_cursor(
    mut mouse_query: Query<(&mut Visibility, &mut Transform), With<MouseCursor>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    windows: Res<Windows>,
    ui_state: Res<UiState>
) {
    let coords = get_world_coords(camera_query, windows);
    let (mut visibility, mut transform) = mouse_query.single_mut();

    if coords[0] == 0.0 && coords[1] == 0.0 {
        visibility.is_visible = false;
        return;
    }

    let (offset, scale, start_x, start_y) = compute_coords(ui_state.square_pixels);
    let (mut x, mut y, mut row) = (start_x, start_y, 0.);

    for idx in 0..64 { // 64 squares in a chessboard
        let offset_check = offset * 2.;
        if coords[0] > x - offset_check + ui_state.square_pixels &&
           coords[0] < x + offset_check &&
           coords[1] > y - offset_check &&
           coords[1] < y + offset_check - ui_state.square_pixels
        {
            transform.translation = Vec3::new(x + offset, y - offset, 1.);
            transform.scale = Vec3::new(scale, scale, 0.);
            visibility.is_visible = ui_state.show_mouse_cursor;
            return;
        }
        else {
            visibility.is_visible = false;
        }

        x += ui_state.square_pixels;

        if (idx + 1) % 8 == 0 { // 8 squares per row
            row += 1.0_f32;
            x = start_x;
            y = start_y - (row * ui_state.square_pixels);
        }
    }

}

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Next).with_system(init_cursor))
            .add_system_set(SystemSet::on_update(GameState::Next).with_system(draw_cursor));
    }
}
