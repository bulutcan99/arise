use crate::spawnable::Faction;
use bevy::math::{Quat, Vec2};
use bevy::prelude::Entity;
use bevy_ecs_macros::Event;
use serde::Deserialize;
use strum_macros::{Display, EnumString};

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
	// Enemy(EnemyMobType),
	Ally(AllyMobType),
}

impl MobType {
	/// Returns the faction of the mob (Enemy or Ally).
	pub fn get_faction(&self) -> Faction {
		match self {
			// MobType::Enemy(_) => Faction::Enemy,
			MobType::Ally(_) => Faction::Ally,
		}
	}

	/// Returns the display name of the mob as a `String`.
	pub fn get_name(&self) -> String {
		match self {
			// MobType::Enemy(enemy) => enemy.to_string(),
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
