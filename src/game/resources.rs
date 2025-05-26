use bevy::prelude::*;
use serde::Deserialize;

#[derive(Resource, Deserialize)]
pub struct GameResource {
    /// Maximum possible projectiles for 1 of the player/mobs shots. Mainly kept low for perf and as
    /// a hard cap (along with fire rate) on how much of a "bullet hell" each mob/player creates.
    pub max_player_projectiles: u16,
    /// Maximum possible speed of an entity
    pub max_speed: f32,
    /// Distance between the center of the screen and the player spawn point
    pub player_spawn_distance: f32,
    /// Sprite image size multiplier
    pub sprite_scale: f32,
    /// Threshold to set velocity to zero
    pub stop_threshold: f32,
    /// Range of mouse scanning
    pub scan_range: f32,
    /// Maximum amount of player inputs to the game
    max_players: u8,
}

impl GameResource {
    pub fn get_max_player(&self) -> u8 {
        self.max_players
    }
}
