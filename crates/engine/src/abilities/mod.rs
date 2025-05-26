use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::abilities::shadow_monarch::{
    MonarchFormData, ShadowDashData, ShadowSummonData,
};

pub mod shadow_monarch;

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
    pub player: Entity,
    /// Slot of the ability that was activated
    pub ability_slot_id: AbilitySlotIDComponent,
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

/// Stores the attributes for all abilities in the game.
/// Each ability is tied to its data type and can be used to generate ECS components.
#[derive(Resource, Deserialize)]
pub struct AbilitiesResource {
    // === Slot1Abilities ===
    /// Sung Jin-Woo's resurrection ability.
    pub shadow_summon: ShadowSummonData,

    // === Slot2Abilities ===
    /// Sung Jin-Woo’s shadow dash.
    pub shadow_dash: ShadowDashData,

    // === Slot3Abilities ===
    /// Sung Jin-Woo's ultimate transformation.
    pub monarch_form: MonarchFormData,
}
