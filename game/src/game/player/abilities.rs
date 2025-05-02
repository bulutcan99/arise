use std::collections::HashMap;
use std::hash::Hash;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Abilities that can occupy the first skill slot of a character.
#[derive(Clone, Deserialize, PartialEq, Eq, Hash)]
pub enum SlotOneAbilityType {
    /// Sung Jin-Woo's shadow resurrection ability.
    ShadowSummon,

    /// Fighter character’s quick melee combo.
    BladeFlurry,
}

/// Abilities that can occupy the second skill slot of a character.
#[derive(Clone, Deserialize, PartialEq, Eq, Hash)]
pub enum SlotTwoAbilityType {
    /// Sung Jin-Woo's shadow dash ability.
    ShadowDash,

    /// Fighter character’s quick dash combo.
    DashStrike,
}

/// Ultimate or transformation abilities in slot 3.
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SlotThreeAbilityType {
    /// Sung Jin-Woo's power-up form
    MonarchForm,

    /// Fighter berserker mode
    Rage,
}

/// Hashmaps of ability types to descriptions
/// Used for providing information to user on character selection screen
#[derive(Resource, Deserialize)]
pub struct AbilityDescriptionsResource {
    pub slot_one: HashMap<SlotOneAbilityType, String>,
    pub slot_two: HashMap<SlotTwoAbilityType, String>,
}
