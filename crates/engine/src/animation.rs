use bevy::prelude::*;

/// This event is used for notifying systems when an animation for an entity has been completed
#[derive(Event)]
pub struct AnimationCompletedEvent(pub Entity);
