use crate::spawnable::Faction;
use bevy::prelude::*;
use serde::Deserialize;
use strum_macros::Display;

/// Represents a projectile entity spawned by a weapon or ability.
/// Contains faction to distinguish between friendly and hostile fire.
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display, Copy)]
pub enum ProjectileType {
	Bullet(Faction),
}

impl ProjectileType {
	/// Returns the faction (Ally or Enemy) of the projectile.
	pub fn get_faction(&self) -> Faction {
		match self {
			ProjectileType::Bullet(faction) => faction.clone(),
		}
	}
}

#[derive(Component)]
pub struct ProjectileComponent;