use crate::core::domain::component::attribute::Attribute;
use crate::core::domain::component::combat::Combat;
use crate::core::domain::component::movement::Movement;
use crate::core::domain::component::stat::Stat;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Warrior {
	attributes: Attribute,
	combat: Combat,
	movement: Movement,
	stat: Stat,
}

impl Warrior {
	pub fn new() -> Self {
		Self {
			attributes: Attribute::new(),
			combat: Combat::new(),
			movement: Movement::new(),
			stat: Stat::new(),
		}
	}
}