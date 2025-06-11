use bevy::prelude::States;

/// High-level application state machine.
/// Controls which screen or phase the player is currently in.
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppStates {
    /// Initial state where assets are being loaded (e.g. textures, sounds).
    #[default]
    LoadingAssets,

    GameInit,

    /// Active gameplay state (the main game loop).
    InGame,

    /// Game Over screen shown when the player is defeated.
    GameOver,


    /*

    /// The main menu screen of the game.
    MainMenu,

    /// Victory screen shown when the player completes the game successfully.
    Victory,

     */
}

