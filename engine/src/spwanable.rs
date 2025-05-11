use bevy::prelude::*;
use serde::Deserialize;
use strum_macros::{Display, EnumString};

/// Defines where in the game world a spawnable should appear.
/// Can be an absolute (global) or relative (local) position.
#[derive(Deserialize, Clone, Debug)]
pub enum SpawnPosition {
    /// A global position in world coordinates (e.g. fixed map location)
    Global(Vec2),

    /// A position relative to an entity's transform (e.g. in front of player or enemy)
    Local(Vec2),
}

/// Represents any entity that can be spawned in the game.
/// Includes projectiles and mobs (enemies or allies).
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum SpawnableType {
    Projectile(ProjectileType),
    Mob(MobType),
}

/// Indicates the faction or alignment of an entity.
/// Useful for determining hostility or target filtering.
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display, Copy)]
pub enum Faction {
    Ally,
    Enemy,
}

/// Represents a projectile entity spawned by a weapon or ability.
/// Contains faction to distinguish between friendly and hostile fire.
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display, Copy)]
pub enum ProjectileType {
    Melee(Faction),
    Range(Faction),
}

impl ProjectileType {
    /// Returns the faction (Ally or Enemy) of the projectile.
    pub fn get_faction(&self) -> Faction {
        match self {
            ProjectileType::Melee(faction) => *faction,
            ProjectileType::Range(faction) => *faction,
        }
    }
}

/// Enemy types from the game world.
/// These are hostile mobs spawned through the level or special events.
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
pub enum EnemyMobType {
    Goblin,
    // Future: Orc, Elf, Knight, etc.
}

/// Allied mobs, such as summons or friendly NPCs.
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
pub enum AllyMobType {
    Soldier,
    ShadowSummon,
    // Future: Healer, Mage, etc.
}

/// A general representation of all mobs in the game.
/// Distinguishes between enemy and ally variants.
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum MobType {
    Enemy(EnemyMobType),
    Ally(AllyMobType),
}

impl MobType {
    /// Returns the faction of the mob (Enemy or Ally).
    pub fn get_faction(&self) -> Faction {
        match self {
            MobType::Enemy(_) => Faction::Enemy,
            MobType::Ally(_) => Faction::Ally,
        }
    }

    /// Returns the display name of the mob as a `String`.
    pub fn get_name(&self) -> String {
        match self {
            MobType::Enemy(enemy) => enemy.to_string(),
            MobType::Ally(ally) => ally.to_string(),
        }
    }
}

/// Event to request the spawning of a mob entity.
/// Used by skills (e.g. ShadowSummon) or level logic.
#[derive(Event)]
pub struct SpawnMobEvent {
    /// Type of mob to spawn
    pub mob_type: MobType,
    /// World-space position to spawn the mob
    pub position: Vec2,
    /// Initial rotation of the mob (useful for aiming/facing)
    pub rotation: Quat,
    /// Whether this mob is a boss or elite variant
    pub boss: bool,
    /// Optional summoner entity (e.g. the player who summoned it)
    pub summoned_by: Option<Entity>,
}

/// Tag component that causes an entity to automatically move
/// toward the closest player entity based on proximity and gravity rules.
#[derive(Component)]
pub struct AttractToClosestPlayerComponent;
