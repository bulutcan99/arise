use bevy_ecs_macros::Component;
use crate::states::player::PlayerState;

/// For use in state transitions
#[derive(Component)]
struct PreviousAnimationState(PlayerState);
