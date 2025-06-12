use bevy::math::Vec2;
use bevy::prelude::Entity;
use bevy_ecs_macros::Event;

#[derive(Event, Debug)]
pub struct MoveEvent {
    pub entity: Entity,
    pub direction: Vec2,
}

#[derive(Event, Debug)]
pub struct DashEvent(pub Entity);

#[derive(Event, Debug)]
pub struct LightAttackEvent(pub Entity);
#[derive(Event, Debug)]
pub struct HeavyAttackEvent(pub Entity);

#[derive(Event, Debug)]
pub struct UseSkillEvent {
    pub entity: Entity,
    pub slot: usize,
}