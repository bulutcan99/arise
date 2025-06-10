pub mod events;

use bevy::prelude::*;
use leafwing_input_manager::prelude::{ActionState, InputMap};
use leafwing_input_manager::{Actionlike, InputManagerBundle};
use serde::Deserialize;

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

    /// Light Attack
    LightAttack,

    /// Heavy Attack
    HeavyAttack,

    /// Use the first ability (Slot 1)
    SlotOneAbility,

    /// Use the second ability (Slot 2)
    SlotTwoAbility,

    /// Use the third ability (Slot 3)
    SlotThreeAbility,
}

/// The parsed input/key bindings used for the life of the  entire game. This is read from files/
/// compiled files very early in the game startup since this must exist in the world before we can
/// accept player/user input.
#[derive(Resource, Debug)]
pub struct InputsResource {
    pub player_keyboard: InputMap<PlayerAction>,
}
