use bevy::prelude::{Entity, Vec2};
use bevy_ecs_macros::Event;
use crate::states::player::PlayerState;

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

#[derive(Event, Debug)]
pub struct AnimationChangeEvent {
    pub entity: Entity,
    pub state: PlayerState,
}
