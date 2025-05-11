use bevy::prelude::*;
use serde::Deserialize;
use strum_macros::{Display, EnumString};

#[derive(Deserialize, Clone, Debug)]
pub enum SpawnPosition {
    /// A global position in world coordinates (e.g. absolute position on the map)
    Global(Vec2),

    /// A local position relative to the entity's current transform (e.g. in front of the player)
    Local(Vec2),
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum SpawnableType {
    Projectile(ProjectileType),
    Mob(MobType),
}

/// Factions
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display, Copy)]
pub enum Faction {
    Ally,
    Enemy,
}

/// Type that encompasses all weapon projectiles
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display, Copy)]
pub enum ProjectileType {
    Slash(Faction),
    Bullet(Faction),
}

impl ProjectileType {
    pub fn get_faction(&self) -> Faction {
        match self {
            ProjectileType::Slash(faction) => faction.clone(),
            ProjectileType::Bullet(faction) => faction.clone(),
        }
    }
}

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
    Orc,
    Elf,
    Knight,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum AllyMobType {
    Soldier,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum MobType {
    Enemy(EnemyMobType),
    Ally(AllyMobType),
}

impl MobType {
    pub fn get_faction(&self) -> Faction {
        match self {
            MobType::Enemy(_) => Faction::Enemy,
            MobType::Ally(_) => Faction::Ally,
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            MobType::Enemy(enemy) => enemy.to_string(),
            MobType::Ally(ally) => ally.to_string(),
        }
    }
}

#[derive(Event)]
pub struct SpawnMobEvent {
    /// Type of mob to spawn
    pub mob_type: MobType,
    /// Position to spawn mob
    pub position: Vec2,

    pub rotation: Quat,

    pub boss: bool,
}

#[derive(Component)]
pub struct AttractToClosestPlayerComponent;
