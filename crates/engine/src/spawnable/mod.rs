pub mod projectile;
pub mod mob;

use crate::spawnable::mob::MobType;
use crate::spawnable::projectile::ProjectileType;
use bevy::prelude::*;
use serde::Deserialize;
use strum_macros::Display;

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


/// Tag component that causes an entity to automatically move
/// toward the closest player entity based on proximity and gravity rules.
#[derive(Component)]
pub struct AttractToClosestPlayerComponent;
