use bevy::prelude::Vec2;
use bevy_ecs_macros::Event;

#[derive(Event)]
pub struct MoveEvent {
    pub direction: Vec2,
}

#[derive(Event)]
pub struct DashEvent;

#[derive(Event)]
pub struct LightAttackEvent;
#[derive(Event)]
pub struct HeavyAttackEvent;

#[derive(Event)]
pub struct UseSkillEvent {
    pub slot: usize,
}
