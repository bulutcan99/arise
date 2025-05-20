use bevy::prelude::*;

use crate::input::PlayerInput;

/// Stores the index (likely 0 or 1) of the player that joined an n-player (max 2) game.
#[derive(Event)]
pub struct PlayerJoinEvent {
    pub player_idx: u8,
    pub input: PlayerInput,
}
