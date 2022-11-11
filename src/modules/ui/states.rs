//! States plugin.

/// The Bevy Game State. Each enum member represents the current game state.
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    /// Initial game state. Load assets before doing anything else.
    AssetLoading,

    /// Final game state. Run the rest of Bevy's systems after loading assets.
    Next,
}
