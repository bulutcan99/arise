use bevy::prelude::*;
use serde::Deserialize;

pub mod enemy;

#[derive(Deserialize, Clone, Debug)]
pub enum SpawnPosition {
    /// A global position in world coordinates (e.g. absolute position on the map)
    Global(Vec2),

    /// A local position relative to the entity's current transform (e.g. in front of the player)
    Local(Vec2),
}
