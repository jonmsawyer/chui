//! States plugin.

use bevy::prelude::*;

/// The Bevy Game State. Each enum member represents the current game state.
#[derive(Default, Clone, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    /// Initial game state. Load assets before doing anything else.
    #[default]
    AssetLoading,

    /// Final game state. Run the rest of Bevy's systems after loading assets.
    Next,
}
