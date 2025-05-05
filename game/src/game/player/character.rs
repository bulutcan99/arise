use bevy::prelude::*;
use serde::Deserialize;
use strum_macros::EnumIter;

use super::abilities::{
    SlotOneAbilityType, SlotThreeAbilityType, SlotTwoAbilityType,
};
use crate::game::spawnable::SpawnPosition;

/// The playable character types. To a player, these will have different appearances and abilities.
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
    #[default]
    ShadowMonarch,
}

/// High-level categories representing a character's overall strengths and weaknesses.
/// These are used to provide a quick overview of the character's general performance style.
#[derive(EnumIter)]
/// Offensive capabilities such as damage, fire rate, and critical hits.
pub enum CharacterStatType {
    Offense,

    /// Defensive capabilities including health, armor, dodge, and sustain effects like lifesteal.
    Defense,

    /// Movement-related attributes like speed and character size.
    Mobility,

    /// Progression speed and efficiency, such as experience gain and luck.
    Growth,

    /// Utility and special traits like cooldown reduction, area of effect size, or projectile count.
    Utility,
}

/// Defines all core data required to represent a playable character in the game.
///
/// A `Character` encapsulates a unique playstyle through its movement parameters,
/// combat capabilities, skill slots, and progression traits. This structure is initialized
/// at the start of a run and governs all player stats and systems.
///
/// Inspired by games like *Vampire Survivors* and the *Solo Leveling* universe,
/// characters may possess summoning powers, life-stealing abilities, area-of-effect attacks,
/// and scaling projectile-based combat.
pub struct Character {
    // === Meta ===
    /// Display name of the character.
    pub name: String,

    /// Type or classification of the character (e.g. Player, Boss, Minion).
    pub character_type: CharacterType,

    /// Ability assigned to the first ability slot (optional).
    pub slot_1_ability: Option<SlotOneAbilityType>,

    /// Ability assigned to the second ability slot (optional).
    pub slot_2_ability: Option<SlotTwoAbilityType>,

    /// Ability assigned to the third ability slot (optional).
    pub slot_3_ability: Option<SlotThreeAbilityType>,

    // === Offense ===
    /// Base melee damage dealt per hit.
    pub weapon_damage: u32,

    /// Number of melee attacks per second.
    pub swing_speed: f32,

    /// Maximum distance the melee attack can reach.
    pub reach: f32,

    // === Defense ===
    /// Maximum health points of the character.
    pub health: usize,

    /// Percentage of damage dealt that is converted into health.
    /// For example, 0.1 = 10% lifesteal.
    pub life_steal_percent: f32,

    // === Mobility ===
    /// Acceleration vector representing how quickly the character gains speed.
    pub acceleration: Vec2,

    /// Deceleration vector representing how quickly the character slows down.
    pub deceleration: Vec2,

    /// Maximum movement speed in each direction.
    pub speed: Vec2,

    /// Physical dimensions of the character's collision box (width, height).
    pub collider_dimensions: Vec2,
}
