use bevy::prelude::*;
use serde::Deserialize;
use strum_macros::{Display, EnumString};

// TODO: abilities ve shadow_monarch.rs dosyalari mantigi olcak
pub struct WeaponData {}

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
    Club(ClubType),
    Sword(SwordType),
    Dagger(DaggerType),
    Bow(BowType),
}

/// Club variations used in melee combat.
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
pub enum ClubType {
    WoodenClub,
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

#[derive(Event)]
pub struct WeaponDamageEvent {
    pub damage: u32,
    pub target: Entity,
}

#[derive(Component, Deserialize, Clone)]
pub struct WeaponCooldownComponent {
    /// Stored seperately so that it can used with the player's cooldown multiplier
    /// to set the duration of the cooldown timer
    pub cooldown_time: f32,
    /// Tracks a cooldown for an ability
    pub cooldown_timer: Timer,
}

impl WeaponCooldownComponent {
    pub fn new(cooldown_time: f32) -> Self {
        Self {
            cooldown_time,
            cooldown_timer: Timer::from_seconds(cooldown_time, TimerMode::Once),
        }
    }
}

#[derive(Resource, Deserialize)]
pub struct WeaponsResource {}
