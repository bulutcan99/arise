use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct DamageDealtEvent {
    pub damage: u32,
    pub target: Entity,
}
