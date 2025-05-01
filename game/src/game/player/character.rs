use serde::Deserialize;
use strum_macros::EnumIter;

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
