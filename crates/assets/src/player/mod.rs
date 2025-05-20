use bevy::prelude::*;

use crate::player::shadow::PlayerShadowAssets;

pub mod shadow;

// -----------------------------------------------------------------------------
// Top-Level Player Asset Container
// -----------------------------------------------------------------------------

/// Top-level container for all player-related assets.
#[derive(Resource)]
pub struct PlayerAssets {
    /// Assets related to the shadow player's animations.
    pub shadow: PlayerShadowAssets,
}
