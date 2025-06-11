use bevy_ecs_macros::{Component, Resource};

use crate::states::player::PlayerState;

#[derive(Component)]
pub struct AnimationStateMachine {
    pub current: PlayerState,
    pub previous: Option<PlayerState>,
}

impl AnimationStateMachine {
    pub fn set(&mut self, new_state: PlayerState) -> bool {
        if self.current != new_state {
            self.previous = Some(self.current);
            self.current = new_state;
            true
        } else {
            false
        }
    }

    pub fn changed(&self) -> bool {
        self.previous != Some(self.current)
    }
}
