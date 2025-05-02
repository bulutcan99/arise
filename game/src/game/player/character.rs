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
    Figther,
}

/// High-level categories representing a character's overall strengths and weaknesses.
/// These are used to provide a quick overview of the character's general performance style.
#[derive(EnumIter)]
pub enum CharacterStatType {
    /// Offensive capabilities such as damage, fire rate, and critical hits.
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
#[derive(Deserialize, Clone)]
pub struct Character {
    // === Meta ===
    pub name: String,
    pub character_type: CharacterType,
    pub slot_1_ability: Option<SlotOneAbilityType>,
    pub slot_2_ability: Option<SlotTwoAbilityType>,
    pub slot_3_ability: Option<SlotThreeAbilityType>,

    // === Offense ===
    pub weapon_damage: usize,
    pub weapon_critical_chance: f32,
    pub weapon_critical_damage: f32,
    pub projectile_speed: f32,
    pub projectile_count: usize,
    pub projectile_size: f32,
    pub projectile_despawn_time: f32,
    pub projectile_spawn_position: SpawnPosition,
    pub area_of_effect: f32,
    pub collision_damage: usize,
    pub summon_damage_multiplier: f32,

    // === Defense ===
    pub health: usize,
    pub shields: usize,
    pub shields_recharge_rate: f32,
    pub dodge_chance: f32,
    pub knockback_resistance: f32,
    pub life_steal_percent: f32,

    // === Mobility ===
    pub acceleration: Vec2,
    pub deceleration: Vec2,
    pub speed: Vec2,
    pub collider_dimensions: Vec2,

    // === Growth ===
    pub xp_gain_rate: f32,
    pub luck: f32,
    pub money: usize,

    // === Utility ===
    pub attraction_distance: f32,
    pub attraction_acceleration: f32,
    pub cooldown_multiplier: f32,
    pub max_summons: usize,
}
