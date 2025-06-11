use bevy_ecs_macros::{Component, Resource};

use crate::states::player::PlayerState;

#[derive(Component)]
pub struct AnimationStateMachine {
    pub current: PlayerState,
    pub previous: Option<PlayerState>,
}

impl AnimationStateMachine {
    pub fn set(&mut self, new: PlayerState) {
        self.previous = Some(self.current);
        self.current = new;
    }
}
