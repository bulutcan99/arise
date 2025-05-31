use bevy::prelude::Entity;
use bevy_ecs_macros::{Component, Event};
use serde::Deserialize;

#[derive(
    Component,
    PartialEq,
    Clone,
    Copy,
    Debug,
    Default,
    Hash,
    Eq,
    Deserialize
)]
pub enum AnimationState {
    #[default]
    Idle,
    Running,
}

#[derive(Event)]
pub struct AnimationChangeEvent {
    pub entity: Entity,
    pub state: AnimationState,
}
