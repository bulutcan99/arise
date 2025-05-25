use bevy::prelude::{Component, States, SystemSet};

/// High-level application state machine.
/// Controls which screen or phase the player is currently in.
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum AppStates {
	/// Initial state where assets are being loaded (e.g. textures, sounds).
	#[default]
	LoadingAssets,

	/// The main menu screen of the game.
	MainMenu,

	/// The screen where the player selects their character before starting a run.
	CharacterSelection,

	/// One-time state to set up the game world before transitioning to gameplay.
	InitializeRun,

	/// Active gameplay state (the main game loop).
	Game,

	/// Game Over screen shown when the player is defeated.
	GameOver,

	/// Victory screen shown when the player completes the game successfully.
	Victory,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameEnterSet {
	Initialize,
	BuildLevel,
	SpawnPlayer,
	BuildUi,
}

/// In-game substates used during active gameplay.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, States)]
pub enum GameStates {
	/// The default state where the game is actively running.
	#[default]
	Playing,

	/// A paused state, typically activated via pause menu or input.
	Paused,
}

// === Cleanup Marker Components ===

/// Tag component used to mark entities that should be removed when leaving the Main Menu state.
#[derive(Component)]
pub struct MainMenuCleanup;

/// Tag component used to mark entities that should be removed when leaving the Game state.
#[derive(Component)]
pub struct GameCleanup;

/// Tag component used to mark entities that should be removed when leaving the Game Over screen.
#[derive(Component)]
pub struct GameOverCleanup;

/// Tag component used to mark entities that should be removed when leaving the Victory screen.
#[derive(Component)]
pub struct VictoryCleanup;

/// Tag component used to mark entities that should be removed when exiting the Pause state.
#[derive(Component)]
pub struct PauseCleanup;

/// Tag component used to mark entities that should be removed when exiting the Character Selection screen.
#[derive(Component)]
pub struct CharacterSelectionCleanup;
