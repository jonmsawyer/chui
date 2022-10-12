//! Module for controlling the game's camera
//!
//! Used by permission from Travis Veazey <https://github.com/Kromey>

use bevy::prelude::*;

use super::super::GameState;

/// Component to attach to the camera we're controlling
///
/// Doing this allows us to easily query for it, while also allowing for cameras to
/// exist that we aren't controlling. This is also extensible, as we can add
/// configurable options on a per-camera basis in the future.
#[derive(Component, Default)]
pub struct MainCamera;

/// Event when we pan the camera, containing the delta of the move
pub struct CameraPanned(Vec2);


/// Set up our 2D orthographic camera
fn setup_camera(mut commands: Commands) {
    // Create an orthgraphic camera and center it on our map
    let camera = Camera2dBundle::default();

    // Position our camera in the center of the world
    //camera.transform.translation = WorldPoint::center().as_vec3(camera.transform.translation.z);

    commands.spawn_bundle(camera).insert(MainCamera::default());
}

/// System to pan our camera
fn pan_camera(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut query: Query<(&mut OrthographicProjection, &mut Transform), With<MainCamera>>,
    mut last_pos: Local<Option<Vec2>>,
    mut pan_evt: EventWriter<CameraPanned>,
) {
    // We only pan the camera on right mouse click
    // NB: If we want to make this configurable, a Vec<MouseButton> in MainCamera could
    // contain a list of any buttons, and .iter().any() used to test
    if !buttons.pressed(MouseButton::Right) {
        // Need to clear our last_pos, otherwise it's just where we were when we
        // last stopped panning, and that causes the camera to just jump around!
        *last_pos = None;
        return;
    }

    let window = match windows.get_primary() {
        Some(window) => window,
        None => return, // Couldn't get the primary window, maybe the game is closing
    };

    // Use cursor position instead of MouseMotion to get acceleration movement
    let current_pos = match window.cursor_position() {
        Some(pos) => pos, // current cursor position
        None => return,   // cursor is not in our window, nothing more we need to be doing now
    };

    // If we didn't have a last_pos, we can't have a delta
    let delta = current_pos - last_pos.unwrap_or(current_pos);
    // If we did have a lost_pos, send the camera pan delta
    if last_pos.is_some() {
        pan_evt.send(CameraPanned(delta));
    }

    for (_projection, mut transform) in query.iter_mut() {
        transform.translation -= delta.extend(0.);
    }

    // Now update our last_pos
    *last_pos = Some(current_pos);
}

/// Our camera controller plugin
pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CameraPanned>()
            .add_startup_system(setup_camera)
            .add_system_set(
                SystemSet::on_update(GameState::Next)
                    .with_system(pan_camera)
            );
    }
}
