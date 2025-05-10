use bevy::prelude::*;
use serde::Deserialize;
use strum_macros::EnumIter;

use super::abilities::{
    SlotOneAbilityType, SlotThreeAbilityType, SlotTwoAbilityType,
};
use crate::spwanable::SpawnPosition;

/// Represents all playable character types in the game.
///
/// Each character type defines unique stats, abilities, and visuals.
/// This is primarily used for character selection and data retrieval.
#[derive(
    Deserialize,
    Clone,
    Debug,
    Hash,
    PartialEq,
    Eq,
    EnumIter,
    Default,
    Copy
)]
pub enum CharacterType {
    /// Default playable character inspired by the Solo Leveling universe.
    #[default]
    ShadowMonarch,
}

/// High-level stat categories used to describe a character's overall strengths.
///
/// These categories help quickly communicate a character's focus or archetype.
#[derive(EnumIter)]
pub enum CharacterStatType {
    /// Offensive capabilities such as damage, fire rate, and critical chance.
    Offense,

    /// Special or utility traits such as skills and projectile count.
    Utility,

    /// Defensive stats like health, armor, dodge rate, and lifesteal.
    Defense,

    /// Movement traits such as speed, agility, and character size.
    Mobility,
}

/// Core data definition for a playable character in the game.
///
/// Each `Character` encapsulates its own movement behavior, combat stats,
/// assigned abilities, and unique traits. This data is used during player setup
/// and governs gameplay systems throughout a run.
///
/// Inspired by games like *Vampire Survivors* and the *Solo Leveling* universe.
pub struct Character {
    // === Meta ===
    /// Display name of the character (for UI and selection screen).
    pub name: String,

    /// Character classification type.
    pub character_type: CharacterType,

    // === Offense ===
    /// Damage dealt when the player collides directly with an enemy.
    pub collision_damage: u32,

    /// Base damage dealt via weapon abilities (e.g. ranged projectiles, AoEs).
    pub weapon_damage: u32,

    /// Speed at which spawned projectiles travel (units per second).
    pub projectile_speed: f32,

    /// Initial offset or direction from which projectiles spawn.
    pub projectile_spawn_position: SpawnPosition,

    /// Lifetime of a projectile in seconds before it is despawned.
    pub projectile_despawn_time: f32,

    /// Size of the projectile for rendering and collision.
    pub projectile_size: f32,

    /// Number of projectiles emitted per attack cycle.
    pub projectile_count: u32,

    // === Abilities ===
    /// Assigned ability for the first active skill slot (if any).
    pub slot_1_ability: Option<SlotOneAbilityType>,

    /// Assigned ability for the second active skill slot (if any).
    pub slot_2_ability: Option<SlotTwoAbilityType>,

    /// Assigned ability for the third active skill slot (if any).
    pub slot_3_ability: Option<SlotThreeAbilityType>,

    // === Defense ===
    /// Total health points for the character.
    pub health: u32,

    /// Percentage of outgoing damage converted back as health.
    /// For example, `0.1` = 10% lifesteal.
    pub life_steal_percent: f32,

    // === Mobility ===
    /// Acceleration applied when movement input is given.
    pub acceleration: Vec2,

    /// Deceleration applied when movement input stops.
    pub deceleration: Vec2,

    /// Maximum speed vector the character can reach.
    pub speed: Vec2,

    /// Physical size used in collision detection (width, height).
    pub collider_dimensions: Vec2,
}
