use bevy::prelude::*;

/// A plugin that manages the internal gameplay state (e.g., Playing vs Paused).
///
/// `GameStatePlugin` sets up a simple state machine to control whether the game
/// is actively running (`Playing`) or temporarily halted (`Paused`).
///
/// This is useful for features like:
/// - Pausing the game when the player presses a key
/// - Freezing gameplay logic while allowing pause menus or overlays
///
/// # How it works
/// - Initializes the `GameState` state machine.
/// - Provides `pause` and `unpause` systems to toggle the state by pressing `P`.
/// - External systems can query the current `GameState` to adapt behavior accordingly.
///
/// # Usage
/// Add the `GameStatePlugin` to your app:
///
/// ```rust
/// app.add_plugins(GameStatePlugin);
/// ```
///
/// Then, run systems conditionally based on the `GameState`, e.g.:
///
/// ```rust
/// app.add_systems(
///     Update,
///     your_gameplay_system.run_if(in_state(GameState::Playing)),
/// );
/// ```
///
/// # Controls
/// - Press `P` to toggle between `Playing` and `Paused`.
///
/// # States
/// - `Playing`: The game is actively running.
/// - `Paused`: The game is frozen; gameplay logic should not update.
///
/// # Notes
/// - Requires `ButtonInput<KeyCode>` resource for key handling (added by Bevy input plugin).
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
    }
}

#[derive(States, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
}

pub fn pause(
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        next_state.set(GameState::Paused);
    }
}

pub fn unpause(
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::KeyP) {
        next_state.set(GameState::Playing);
    }
}
