use assets::player::shadow::PlayerShadowAssets;
use bevy::log::{debug, info, warn};
use bevy::prelude::{
    Entity, EventReader, Query, Res, Sprite, TextureAtlas, Timer, With,
};
use engine::animation::AnimationComponent;
use engine::events::animation::AnimationChangeEvent;
use engine::player::PlayerComponent;
use engine::states::animation::AnimationStateMachine;
use engine::states::player::PlayerState;

use crate::animation::animation::AnimationsResource;

pub fn player_animation_controller_system(
    mut events: EventReader<AnimationChangeEvent>,
    animations_resource: Res<AnimationsResource>,
    player_assets: Res<PlayerShadowAssets>,
    mut query: Query<
        (
            &mut AnimationComponent,
            &mut Sprite,
            &mut AnimationStateMachine,
        ),
        With<PlayerComponent>,
    >,
) {
    for event in events.read() {
        let Ok((
            mut anim_component,
            mut sprite,
            mut state_machine,
        )) = query.get_mut(event.entity) else {
            warn!("Failed to get components for AnimationChangeEvent on entity {:?}", event.entity);
            continue;
        };

        if !state_machine.set(event.state) {
            debug!(
                "Animation state {:?} already active.",
                event.state
            );
            continue;
        }

        if let Some(new_anim_data) =
            animations_resource.animations.get(&event.state)
        {
            anim_component.timer = Timer::from_seconds(
                new_anim_data.frame_duration,
                new_anim_data.mode.into(),
            );
            anim_component.direction = new_anim_data.direction;
            anim_component.mode = new_anim_data.mode;

            let (image, layout) = match event.state {
                PlayerState::Idle => {
                    (
                        player_assets.idle_image.clone(),
                        player_assets.idle_layout.clone(),
                    )
                },
                PlayerState::Running => {
                    (
                        player_assets.run_image.clone(),
                        player_assets.run_layout.clone(),
                    )
                },
                PlayerState::LightAttack => {
                    (
                        player_assets.light_attack_image.clone(),
                        player_assets.light_attack_layout.clone(),
                    )
                },
            };

            *sprite =
                Sprite::from_atlas_image(image, TextureAtlas::from(layout));
            anim_component.timer.reset();
            anim_component.timer.unpause();
        } else {
            warn!(
                "Animation data not found for state {:?}",
                event.state
            );
        }
    }
}
