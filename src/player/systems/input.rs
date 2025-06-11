use bevy::prelude::*;
use engine::events::{
    AnimationChangeEvent, DashEvent, HeavyAttackEvent, LightAttackEvent,
    MoveEvent, UseSkillEvent,
};
use engine::input::PlayerAction;
use engine::player::PlayerComponent;
use engine::states::player::{try_set_player_state, PlayerState};
use leafwing_input_manager::prelude::ActionState;
pub fn player_input_router_system(
    mut action_state_query: Query<(
        Entity,
        &ActionState<PlayerAction>,
        Mut<PlayerState>,
    ), With<PlayerComponent>>,
    mut move_event_writer: EventWriter<MoveEvent>,
    mut dash_event_writer: EventWriter<DashEvent>,
    mut light_attack_writer: EventWriter<LightAttackEvent>,
    mut heavy_attack_writer: EventWriter<HeavyAttackEvent>,
    mut skill_writer: EventWriter<UseSkillEvent>,
    mut animation_events: EventWriter<AnimationChangeEvent>,
) {

    let Ok((entity, action_state, mut current_player_state)) = action_state_query.get_single_mut() else {
        error!("Player action state not found.");
        return;
    };

    let mut direction = Vec2::ZERO;
    if action_state.pressed(&PlayerAction::MoveUp) { direction.y += 1.0; }
    if action_state.pressed(&PlayerAction::MoveDown) { direction.y -= 1.0; }
    if action_state.pressed(&PlayerAction::MoveRight) { direction.x += 1.0; }
    if action_state.pressed(&PlayerAction::MoveLeft) { direction.x -= 1.0; }

    let mut player_states = vec![];
    if direction.length_squared() > 0.0 {
        player_states.push(PlayerState::Running);
    }
    if action_state.just_pressed(&PlayerAction::LightAttack) {
        player_states.push(PlayerState::LightAttack);
    }

    /*
    Not implemented
    if action_state.just_pressed(&PlayerAction::Dash) {
        player_states.push(PlayerState::Dashing);
    }
    if action_state.just_pressed(&PlayerAction::HeavyAttack) {
        player_states.push(PlayerState::HeavyAttack);
    }
    if action_state.just_pressed(&PlayerAction::SlotOneAbility) {
        player_states.push(PlayerState::Casting);
    }
    if action_state.just_pressed(&PlayerAction::SlotTwoAbility) {
        player_states.push(PlayerState::Casting);
    }
    if action_state.just_pressed(&PlayerAction::SlotThreeAbility) {
        player_states.push(PlayerState::Casting);
    }
    */

    if player_states.is_empty() {
        player_states.push(PlayerState::Idle);
    }

    // 2. En öncelikli state’i seç
    if let Some(new_state) = player_states.into_iter().max_by_key(|s| s.priority()) {
        // 3. Eğer state değiştiyse:
        if new_state != *current_player_state {
            *current_player_state = new_state;

            // 4. Animasyonu tetikle
            info!("New State {:?}", new_state);
            animation_events.send(AnimationChangeEvent {
                entity,
                state: new_state.into(), // Eğer AnimationState’e dönüşüyorsa
            });

            // 5. Davranış Event’ini tetikle
            match new_state {
                PlayerState::Running => {
                    move_event_writer.send(MoveEvent {
                        entity,
                        direction: direction.normalize_or_zero(),
                    });
                }
                PlayerState::LightAttack => {
                    light_attack_writer.send(LightAttackEvent(entity));
                }
                /*
                Not implemented!
                PlayerState::HeavyAttack => {
                    heavy_attack_writer.send(HeavyAttackEvent(entity));
                }
                PlayerState::Dashing => {
                    dash_event_writer.send(DashEvent(entity));
                }
                PlayerState::Casting => {
                    // Slot bilgisi eksik olduğu için burada tahmini 1 yazdım.
                    skill_writer.send(UseSkillEvent { entity, slot: 1 });
                }
                */
                _ => {
                    info!("Player state changed to {:?}", new_state);
                }
            }
        }
    }
}
