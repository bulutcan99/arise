use bevy::log::{debug, info};
use bevy::prelude::{Entity, EventWriter, Query, Res, Time, Transform, With};
use engine::animation::states::{AnimationChangeEvent, AnimationState};
use engine::input::PlayerAction;
use engine::player::{
    PlayerComponent, PlayerMobilityComponent, PlayerVelocityComponent,
};
use leafwing_input_manager::action_state::ActionState;

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
            &AnimationState,
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
        let is_attack_pressed =
            action_state.pressed(&PlayerAction::LightAttack);

        if is_attack_pressed
            && *current_animation_state != AnimationState::LightAttack
        {
            animation_events.send(AnimationChangeEvent {
                entity,
                state: AnimationState::LightAttack,
            });
            info!("Light attack started (pressed)");
        }

        if !is_attack_pressed
            && *current_animation_state == AnimationState::LightAttack
        {
            // Opsiyonel: saldırı tuşu bırakıldığında geri idle/run vs. yapılacaksa buraya yazılır.
            debug!("Light attack input released");
        }
    }
}
