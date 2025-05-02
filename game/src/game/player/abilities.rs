use serde::{Deserialize, Serialize};

/// Abilities that can occupy the first skill slot of a character.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum SlotOneAbilityType {
    /// Sung Jin-Woo's shadow resurrection ability.
    ShadowSummon,

    /// Fighter character’s quick melee combo.
    BladeFlurry,
}

/// Abilities that can occupy the second skill slot of a character.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum SlotTwoAbilityType {
    /// Sung Jin-Woo's shadow dash ability.
    ShadowDash,

    /// Fighter character’s quick dash combo.
    DashStrike,
}

/// Ultimate or transformation abilities in slot 3.
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum SlotThreeAbilityType {
    /// Sung Jin-Woo's power-up form
    MonarchForm,

    /// Fighter berserker mode
    Rage,
}
