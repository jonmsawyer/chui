//! Game State plugin

use super::super::states::GameState;
use bevy::prelude::*;

/// Game state Bevy plugin.
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>();
    }
}
