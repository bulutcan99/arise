use bevy::prelude::*;
use serde::Deserialize;

use super::abilities::{AbilityCooldownComponent, AbilitySlotIDComponent};

// === Ability1 ===
/// Bundle used for spawning the shadow summon ability entity as part of a player or system.
/// Groups all related components together for simplified spawning.
#[derive(Bundle, Clone)]
pub struct ShadowSummonBundle {
    /// Slot identifier (e.g., Slot1, Slot2) used for ability mapping
    pub slot: AbilitySlotIDComponent,

    /// Cooldown tracker for the ability
    pub cooldown: AbilityCooldownComponent,

    /// Core logic and configuration for the shadow summon behavior
    pub ability: ShadowSummonComponent,
}

impl From<&ShadowSummonData> for ShadowSummonBundle {
    fn from(data: &ShadowSummonData) -> Self {
        Self {
            slot: data.slot,
            cooldown: AbilityCooldownComponent::new(data.cooldown_time),
            ability: ShadowSummonComponent::from(data.ability),
        }
    }
}

/// Deserializable data for initializing a `ShadowSummonBundle`.
/// Used during asset loading or runtime configuration.
#[derive(Deserialize, Clone, Copy, Debug)]
pub struct ShadowSummonData {
    pub slot: AbilitySlotIDComponent,
    pub cooldown_time: f32,
    pub ability: ShadowSummonComponentData,
}

/// Component responsible for tracking summon-related gameplay logic.
/// This component handles stats and timers of active summons derived from defeated enemies.
#[derive(Component, Clone, Debug)]
pub struct ShadowSummonComponent {
    /// Maximum number of active summons
    pub max_summons: u32,

    /// Maximum radius that summons can go
    pub max_radius: f32,

    /// How much health is inherited by the summon (0.0 to 1.0)
    pub health_percentage: f32,

    /// How much damage is inherited by the summon (0.0 to 1.0)
    pub damage_percentage: f32,

    /// Optional timer tracking summon lifetime. If None, summon is permanent.
    pub lifetime: Option<Timer>,
}

impl From<ShadowSummonComponentData> for ShadowSummonComponent {
    fn from(data: ShadowSummonComponentData) -> Self {
        Self {
            max_summons: data.max_summons,
            max_radius: data.max_radius,
            health_percentage: data.health_percentage,
            damage_percentage: data.damage_percentage,
            lifetime: data
                .duration
                .map(|secs| Timer::from_seconds(secs, TimerMode::Once)),
        }
    }
}

/// Deserializable data for configuring a `ShadowSummonComponent`.
/// Defines logic for resurrecting defeated enemies into summons.
#[derive(Deserialize, Clone, Copy, Debug)]
pub struct ShadowSummonComponentData {
    pub max_summons: u32,
    pub max_radius: f32,
    pub health_percentage: f32,
    pub damage_percentage: f32,
    pub duration: Option<f32>,
}

// === Ability2 ===
/// Bundle used to spawn and configure a dash ability entity.
/// Includes metadata like cooldown and slot assignment.
pub struct ShadowDashBundle {
    /// Which ability slot this ability occupies (e.g., Slot1, Slot2).
    pub slot: AbilitySlotIDComponent,
    /// Cooldown component storing internal timer state.
    pub cooldown: AbilityCooldownComponent,
    /// Actual dash behavior and parameters.
    pub ability: ShadowDashComponent,
}

impl From<&ShadowDashData> for ShadowDashBundle {
    fn from(data: &ShadowDashData) -> Self {
        Self {
            slot: data.slot,
            cooldown: AbilityCooldownComponent::new(data.cooldown_time),
            ability: ShadowDashComponent::from(&data.ability),
        }
    }
}

/// Deserializable configuration data for a full dash ability.
/// This includes slot placement, cooldown, and behavior.
#[derive(Deserialize, Clone, Debug)]
pub struct ShadowDashData {
    pub slot: AbilitySlotIDComponent,
    pub cooldown_time: f32,
    pub ability: ShadowDashComponentData,
}

/// Component representing the active dash state of a shadow entity.
/// Applies a temporary movement boost in a specified direction.
#[derive(Component)]
pub struct ShadowDashComponent {
    /// Duration (in seconds) the dash effect lasts.
    pub duration: f32,
    /// How much faster the entity moves during the dash (e.g., 2.0 = 2x speed).
    pub speed_multiplier: f32,
    /// Direction of the dash as a normalized 2D vector.
    pub direction: Vec2,
}

impl From<&ShadowDashComponentData> for ShadowDashComponent {
    fn from(data: &ShadowDashComponentData) -> Self {
        Self {
            duration: data.duration,
            speed_multiplier: data.speed_multiplier,
            direction: Vec2::new(data.direction.0, data.direction.1),
        }
    }
}

/// Deserializable configuration data for a `ShadowDashComponent`.
/// Typically loaded from external files (e.g., `.ron` or `.json`).
#[derive(Deserialize, Clone, Copy, Debug)]
pub struct ShadowDashComponentData {
    pub duration: f32,
    pub speed_multiplier: f32,
    pub direction: (f32, f32),
}

// === Ability3 ===
/// Bundle used to spawn a `MonarchForm` ability entity.
/// Combines slot, cooldown tracking, and the core behavior component.
pub struct MonarchFormBundle {
    /// Which ability slot this form occupies.
    pub slot: AbilitySlotIDComponent,
    /// Cooldown timer component for reuse delay.
    pub cooldown: AbilityCooldownComponent,
    /// Actual Monarch Form behavior.
    pub ability: MonarchFormComponent,
}

impl From<&MonarchFormData> for MonarchFormBundle {
    fn from(data: &MonarchFormData) -> Self {
        Self {
            slot: data.slot,
            cooldown: AbilityCooldownComponent::new(data.cooldown_time),
            ability: MonarchFormComponent::from(&data.ability),
        }
    }
}

/// Deserializable configuration data for a complete Monarch Form ability.
/// This includes slot placement, cooldown duration, and form behavior.
#[derive(Deserialize, Clone, Debug)]
pub struct MonarchFormData {
    pub slot: AbilitySlotIDComponent,
    pub cooldown_time: f32,
    pub ability: MonarchFormComponentData,
}

/// Runtime component representing the Monarch Form state.
/// Adds temporary bonuses to health and damage for a limited time.
#[derive(Component, Clone, Copy, Debug)]
pub struct MonarchFormComponent {
    /// Percentage of max health granted.
    pub health_percentage: f32,
    /// Percentage of bonus damage applied.
    pub damage_percentage: f32,
    /// Duration (in seconds) the form lasts.
    pub duration: f32,
}

impl From<&MonarchFormComponentData> for MonarchFormComponent {
    fn from(data: &MonarchFormComponentData) -> Self {
        Self {
            health_percentage: data.health_percentage,
            damage_percentage: data.damage_percentage,
            duration: data.duration,
        }
    }
}

/// Deserializable configuration data for a `MonarchFormComponent`.
/// Typically loaded from external files (e.g., `.ron`, `.json`).
#[derive(Deserialize, Clone, Copy, Debug)]
pub struct MonarchFormComponentData {
    pub health_percentage: f32,
    pub damage_percentage: f32,
    pub duration: f32,
}
