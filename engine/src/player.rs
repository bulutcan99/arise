use bevy::core_pipeline::core_2d::graph::input;
use bevy::prelude::*;

use crate::character::CharacterType;
use crate::input::PlayerInput;

/// Stores all available player slots
#[derive(Resource, Debug, Default)]
pub struct PlayersResource {
    /// Vec of Optional players, an index is Some if a player has joined for that slot
    pub player_data: Vec<Option<PlayerData>>,
}

impl PlayersResource {
    pub fn get_all_used_inputs(&self) -> Vec<PlayerInput> {
        self.player_data
            .iter()
            .filter_map(|valid_data| valid_data.clone().map(|data| data.input))
            .collect()
    }
}

/// Information about a player slot
#[derive(Debug, Clone)]
pub struct PlayerData {
    /// The character that a joined player has chosen
    pub character: CharacterType,
    /// Input method of a joined player
    pub input: PlayerInput,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    id: PlayerIDComponent,
}

#[derive(Component)]
pub struct PlayerComponent;

/// Identity of a player component, used for syncing UI
#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum PlayerIDComponent {
    One,
    Two,
}
