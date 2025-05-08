use std::env::current_dir;
use std::fs::read_to_string;

use bevy::prelude::*;
use leafwing_input_manager::prelude::{ActionState, InputMap};
use leafwing_input_manager::{Actionlike, InputManagerBundle};
use ron::from_str;
use serde::Deserialize;

/// Spawns entity to track navigation over menus
pub fn spawn_menu_explorer_system(
    mut commands: Commands,
    inputs_res: Res<InputsResource>,
) {
    commands
        .spawn(InputManagerBundle::<MenuAction> {
            action_state: ActionState::default(),
            input_map: inputs_res.menu.clone(),
        })
        .insert(MainMenuExplorer);
}

/// Represents the type of input device a player is using.
#[derive(Clone, PartialEq, Debug, Copy)]
pub enum PlayerInput {
    /// Keyboard input
    Keyboard,

    /// Gamepad or controller input
    Gamepad,
}

/// Identifies a specific player's menu input context.
/// Used to route input to the correct UI element in local multiplayer menus.
#[derive(Component)]
pub struct MenuExplorer;

/// Shared UI explorer for accessing global menus such as the main menu or pause screen.
/// Only one instance should exist and it is not player-specific.
#[derive(Component)]
pub struct MainMenuExplorer;

/// Defines input actions that are available in menu contexts.
/// Includes joining, navigating, confirming, and pausing, with device-specific variants.
#[derive(
    Actionlike,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    Debug,
    Reflect,
    Deserialize
)]
pub enum MenuAction {
    /// Confirm or select a menu item
    Confirm,

    /// Join the game using a keyboard
    JoinKeyboard,

    /// Join the game using a gamepad
    JoinGamepad,

    /// Go back or cancel an action
    Back,

    /// Reset the menu state or selection
    Reset,

    /// Exit the pause menu
    ExitPauseMenu,

    /// Pause the game (usually mapped to a specific key/button)
    PauseGame,

    // Navigation - Keyboard
    /// Navigate up in the menu using keyboard input
    NavigateUpKeyboard,

    /// Navigate down in the menu using keyboard input
    NavigateDownKeyboard,

    /// Navigate left in the menu using keyboard input
    NavigateLeftKeyboard,

    /// Navigate right in the menu using keyboard input
    NavigateRightKeyboard,

    // Navigation - Gamepad
    /// Navigate up in the menu using gamepad input
    NavigateUpGamepad,

    /// Navigate down in the menu using gamepad input
    NavigateDownGamepad,

    /// Navigate left in the menu using gamepad input
    NavigateLeftGamepad,

    /// Navigate right in the menu using gamepad input
    NavigateRightGamepad,

    /// Ready up using keyboard input
    PlayerReadyKeyboard,

    /// Ready up using gamepad input
    PlayerReadyGamepad,
}

/// Defines input actions available during gameplay (e.g., when fighting enemies).
/// These actions can be triggered using both keyboard and gamepad inputs.
#[derive(
    Actionlike,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Hash,
    Debug,
    Reflect,
    Deserialize
)]
pub enum PlayerAction {
    /// Move the character upward
    MoveUp,

    /// Move the character downward
    MoveDown,

    /// Move the character to the left
    MoveLeft,

    /// Move the character to the right
    MoveRight,

    /// Dash
    Dash,

    /// Use the first ability (Slot 1)
    SlotOneAbility,

    /// Use the second ability (Slot 2)
    SlotTwoAbility,

    /// Use the third ability (Slot 3)
    SlotThreeAbility,
}

/// Holds the full input mappings used throughout the game for menus and player controls.
/// Loaded during the early startup phase from configuration files or serialized data.
#[derive(Resource, Debug)]
pub struct InputsResource {
    /// Input map for menu-related actions (navigation, selection, etc.)
    pub menu: InputMap<MenuAction>,

    /// Input map for player actions when using a keyboard
    pub player_keyboard: InputMap<PlayerAction>,

    /// Input map for player actions when using a gamepad
    pub player_gamepad: InputMap<PlayerAction>,
}

#[derive(Deserialize)]
pub struct InputData {
    pub menu_keyboard: Vec<(MenuAction, KeyCode)>,
    pub menu_gamepad: Vec<(MenuAction, GamepadButton)>,
    pub player_keyboard: Vec<(PlayerAction, KeyCode)>,
    pub player_gamepad: Vec<(PlayerAction, GamepadButton)>,
    pub player_mouse: Vec<(PlayerAction, MouseButton)>,
}

impl From<InputData> for InputsResource {
    fn from(bindings: InputData) -> Self {
        InputsResource {
            menu: InputMap::new(bindings.menu_keyboard)
                .insert_multiple(bindings.menu_gamepad)
                .to_owned(),
            player_keyboard: InputMap::new(bindings.player_keyboard)
                .insert_multiple(bindings.player_mouse)
                .to_owned(),
            player_gamepad: InputMap::new(bindings.player_gamepad),
        }
    }
}

pub(super) fn get_input_bindings() -> InputData {
    // TODO: input.ron alcak sekilde duzenlencek!
    let config_path = current_dir().unwrap().join("config");

    from_str::<InputData>(
        &read_to_string(config_path.join("input.ron")).unwrap(),
    )
    .unwrap()
}
