// === Cleanup Marker Components ===

use bevy_ecs_macros::Component;

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

