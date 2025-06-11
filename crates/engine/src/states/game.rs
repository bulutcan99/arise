use bevy::prelude::States;

/*
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameEnterSet {
    Initialize,
    BuildLevel,
    SpawnPlayer,
    BuildUi,
}
*/

/// In-game substates used during active gameplay.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, States)]
pub enum GameStates {
    /// The default state where the game is actively running.
    #[default]
    Playing,

    /// A paused state, typically activated via pause menu or input.
    Paused,
}
