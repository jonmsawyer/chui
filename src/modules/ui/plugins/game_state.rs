//! Game State plugin

use bevy::{prelude::*};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetLoading,
    Next,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        //Start off with the default loading state (AssetLoading) and then
        // once the AssetLoading is finished, moved onto the Next state.
        app.add_state(GameState::AssetLoading);
    }
}
