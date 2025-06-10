use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use engine::input::events::{DashEvent, HeavyAttackEvent, LightAttackEvent, MoveEvent, UseSkillEvent};
use engine::input::PlayerAction;
use engine::player::PlayerComponent;

pub fn player_input_router_system(
    action_state: Query<&ActionState<PlayerAction>, With<PlayerComponent>>,
    mut move_event_writer: EventWriter<MoveEvent>,
    mut dash_event_writer: EventWriter<DashEvent>,
    mut light_attack_writer: EventWriter<LightAttackEvent>,
    mut heavy_attack_writer: EventWriter<HeavyAttackEvent>,
    mut skill_writer: EventWriter<UseSkillEvent>,
) {
    let action_state = match action_state.get_single() {
        Ok(state) => state,
        Err(_) => {
            error!("Action state doesn't exist");
            return},
    };

    // Movement
    let mut direction = Vec2::ZERO;
    if action_state.pressed(&PlayerAction::MoveUp) {
        direction.y += 1.0;
    }
    if action_state.pressed(&PlayerAction::MoveDown) {
        direction.y -= 1.0;
    }
    if action_state.pressed(&PlayerAction::MoveRight) {
        direction.x += 1.0;
    }
    if action_state.pressed(&PlayerAction::MoveLeft) {
        direction.x -= 1.0;
    }
    if direction != Vec2::ZERO {
        move_event_writer.send(MoveEvent {
            direction: direction.normalize(),
        });
    }

    if action_state.just_pressed(&PlayerAction::Dash) {
        dash_event_writer.send(DashEvent);
    }

    // Attack
    if action_state.just_pressed(&PlayerAction::LightAttack) {
        light_attack_writer.send(LightAttackEvent);
    }
    if action_state.just_pressed(&PlayerAction::HeavyAttack) {
        heavy_attack_writer.send(HeavyAttackEvent);
    }

    // Skill
    if action_state.just_pressed(&PlayerAction::SlotOneAbility) {
        skill_writer.send(UseSkillEvent { slot: 1 });
    }
    if action_state.just_pressed(&PlayerAction::SlotTwoAbility) {
        skill_writer.send(UseSkillEvent { slot: 2 });
    }
    if action_state.just_pressed(&PlayerAction::SlotThreeAbility) {
        skill_writer.send(UseSkillEvent { slot: 3 });
    }
}
