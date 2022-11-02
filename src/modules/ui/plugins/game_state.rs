//! Game State plugin

use bevy::prelude::*;
use super::super::states::GameState;


pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        // Start off with the default loading state (AssetLoading) and then
        // once the AssetLoading is finished, moved onto the Next state.
        app.add_state(GameState::AssetLoading);
    }
}
