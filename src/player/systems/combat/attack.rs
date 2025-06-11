use bevy::log::{debug, info};
use bevy::prelude::{Entity, EventWriter, Query, Res, Time, Transform, With};
use engine::input::PlayerAction;
use engine::player::{
    PlayerComponent, PlayerMobilityComponent, PlayerVelocityComponent,
};
use leafwing_input_manager::action_state::ActionState;
use engine::events::AnimationChangeEvent;
use engine::states::player::PlayerState;
use crate::game::resources::GameResource;
pub fn light_attack_system(
    time: Res<Time>,
    game_parameters: Res<GameResource>,
    mut animation_events: EventWriter<AnimationChangeEvent>,
    mut player_query: Query<
        (
            Entity,
            &PlayerMobilityComponent,
            &mut Transform,
            &ActionState<PlayerAction>,
            &PlayerState,
            &mut PlayerVelocityComponent,
        ),
        With<PlayerComponent>,
    >,
) {
    for (
        entity,
        _player_mobility,
        _mut_transform,
        action_state,
        current_animation_state,
        _mut_player_velocity,
    ) in player_query.iter_mut()
    {
        debug!("{:?}", action_state);
    }
}
