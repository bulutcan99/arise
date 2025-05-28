use bevy::prelude::Entity;
use bevy_ecs_macros::Event;

#[derive(Event)]
pub struct DamageDealtEvent{
    pub target: Entity,
    pub damage: u32,
}

/// Event to notify that a specific entity's health regeneration should reset.
/// Typically emitted when the entity takes damage.
#[derive(Event)]
pub struct HealthRegainResetEvent {
    pub entity: Entity,
}