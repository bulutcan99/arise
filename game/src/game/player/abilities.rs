use std::collections::HashMap;
use std::hash::Hash;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::PlayerIDComponent;

#[derive(Component, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum AbilitySlotIDComponent {
    One,
    Two,
    Three,
}

/// Abilities that can occupy the first skill slot of a character.
#[derive(Clone, Deserialize, PartialEq, Eq, Hash)]
pub enum SlotOneAbilityType {
    /// Sung Jin-Woo's shadow resurrection ability.
    ShadowSummon,
}

/// Abilities that can occupy the second skill slot of a character.
#[derive(Clone, Deserialize, PartialEq, Eq, Hash)]
pub enum SlotTwoAbilityType {
    /// Sung Jin-Woo's shadow dash ability.
    ShadowDash,
}

/// Ultimate or transformation abilities in slot 3.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SlotThreeAbilityType {
    /// Sung Jin-Woo's power-up form
    MonarchForm,
}

/// Hashmaps of ability types to descriptions
/// Used for providing information to user on character selection screen
#[derive(Resource, Deserialize)]
pub struct AbilityDescriptionsResource {
    pub slot_one: HashMap<SlotOneAbilityType, String>,
    pub slot_two: HashMap<SlotTwoAbilityType, String>,
    pub slot_three: HashMap<SlotThreeAbilityType, String>,
}

/// Event for triggering ability systems to fire when criteria like inputs and cooldowns are met
#[derive(Event, Debug)]
pub struct ActivateAbilityEvent {
    /// ID of the player that activated the ability
    pub player_id: PlayerIDComponent,
    /// Slot of the ability that was activated
    pub ability_slot_id: AbilitySlotIDComponent,
}

impl ActivateAbilityEvent {
    pub fn new(
        player_id: PlayerIDComponent,
        ability_slot_id: AbilitySlotIDComponent,
    ) -> Self {
        Self {
            player_id,
            ability_slot_id,
        }
    }
}

/// Stores the attributes for all abilities in the game.
/// Each ability is tied to its data type and can be used to generate ECS components.
#[derive(Resource, Deserialize)]
pub struct AbilitiesResource {
    // === Ability1 ===
    /// Sung Jin-Woo's resurrection ability.
    pub shadow_summon: ShadowSummonData,

    // === Ability2 ===
    /// Sung Jin-Woo’s shadow dash.
    pub shadow_dash: DashAbilityData,

    // === Ability3 ===
    /// Sung Jin-Woo's ultimate transformation.
    pub monarch_form: MonarchFormData,
}

/// Component for tracking ability cooldowns
#[derive(Component, Deserialize, Clone)]
pub struct AbilityCooldownComponent {
    /// Stored seperately so that it can used with the player's cooldown multiplier
    /// to set the duration of the cooldown timer
    pub cooldown_time: f32,
    /// Tracks a cooldown for an ability
    pub cooldown_timer: Timer,
}

impl AbilityCooldownComponent {
    pub fn new(cooldown_time: f32) -> Self {
        Self {
            cooldown_time,
            cooldown_timer: Timer::from_seconds(cooldown_time, TimerMode::Once),
        }
    }
}

/// Deserializable data for `ShadowSummonBundle`
/// Stores minimum data required to instantiate
pub struct ShadowSummonData {
    /// Which ability slot this ability occupies (Slot1, Slot2, etc.)
    pub slot: AbilitySlotIDComponent,

    /// Cooldown time
    pub cooldown_time: f32,

    /// Core attributes of summon shadow ability
    pub ability: ShadowSummonComponentData,
}

/// Deserializable data for `ShadowSummonComponent`.
/// Defines the resurrection logic for converting defeated enemies into player-controlled summons.
#[derive(Deserialize, Clone, Copy, Debug)]
pub struct ShadowSummonComponentData {
    /// Maximum number of summons that can be active at a time
    pub max_summons: u32,

    /// How much health a summon inherits from the defeated enemy (0.0 to 1.0)
    pub health_percentage: f32,

    /// How much damage a summon inherits from the defeated enemy (0.0 to 1.0)
    pub damage_percentage: f32,

    /// Optional duration the summon remains alive (in seconds). If None, it's permanent.
    pub duration: Option<f32>,
}
