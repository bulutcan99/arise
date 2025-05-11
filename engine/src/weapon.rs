use std::collections::HashMap;

use bevy::prelude::*;
use serde::Deserialize;
use strum_macros::{Display, EnumString};

use crate::spwanable::SpawnPosition;

/// The main weapon type, including subtypes for swords, daggers, and bows.
#[derive(
    Deserialize,
    EnumString,
    Display,
    Debug,
    Hash,
    PartialEq,
    Eq,
    Clone,
    Copy
)]
pub enum WeaponType {
    Sword(SwordType),
    Dagger(DaggerType),
    Bow(BowType),
}

/// Sword variations used in melee combat.
#[derive(
    Deserialize,
    EnumString,
    Display,
    Debug,
    Hash,
    PartialEq,
    Eq,
    Clone,
    Copy
)]
pub enum SwordType {
    Longsword,
}

/// Dagger variations used for fast melee attacks.
#[derive(
    Deserialize,
    EnumString,
    Display,
    Debug,
    Hash,
    PartialEq,
    Eq,
    Clone,
    Copy
)]
pub enum DaggerType {
    ShadowDagger,
}

/// Bow variations used for ranged combat.
#[derive(
    Deserialize,
    EnumString,
    Display,
    Debug,
    Hash,
    PartialEq,
    Eq,
    Clone,
    Copy
)]
pub enum BowType {
    WoodenBow,
}

/// Core weapon statistics such as damage, attack speed, and attack range.
#[derive(Debug, Clone, Reflect)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub damage: f32,
    pub cooldown_seconds: f32,
    pub projectile_spawn_position: SpawnPosition,
    pub projectile_count: u32,
    pub area_size_multiplier: f32,
    pub projectile_speed: f32,
    pub projectile_lifetime_seconds: f32,
    pub pierce_count: u32,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            weapon_type: WeaponType::Sword(SwordType::Longsword),
            damage: 10.0,
            cooldown_seconds: 1.0,
            projectile_count: 1,
            projectile_spawn_position: SpawnPosition::Local(Vec2::Y),
            area_size_multiplier: 1.0,
            projectile_speed: 300.0,
            projectile_lifetime_seconds: 2.0,
            pierce_count: 0,
        }
    }
}

#[derive(Resource, Deserialize)]
pub struct WeaponResource {
    #[serde(flatten)]
    pub all_weapons: HashMap<String, Weapon>,
}

impl WeaponResource {
    pub fn get_stats(&self, weapon_name: &str) -> Option<&Weapon> {
        self.all_weapons.get(weapon_name)
    }
}
