use bevy_ecs_macros::Component;
use serde::Deserialize;

/// Player States
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
pub enum PlayerState {
    #[default]
    Idle,
    Running,
    LightAttack,
}

impl PlayerState {
    pub fn priority(&self) -> u8 {
        match self {
            PlayerState::LightAttack => 3,
            PlayerState::Running => 2,
            PlayerState::Idle => 1,
        }
    }
}

pub fn try_set_player_state(current: &mut PlayerState, new: Vec<PlayerState>) -> bool {
    if let Some(best_candidate) = new.into_iter().max_by_key(|s| s.priority()) {
        if best_candidate.priority() >= current.priority() {
            *current = best_candidate;
            return true;
        }
    }
    false
}
