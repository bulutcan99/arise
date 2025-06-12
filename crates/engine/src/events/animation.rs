use crate::states::player::PlayerState;
use bevy::prelude::Entity;
use bevy_ecs_macros::Event;

#[derive(Event, Debug)]
pub struct AnimationChangeEvent {
    pub entity: Entity,
    pub state: PlayerState,
}
