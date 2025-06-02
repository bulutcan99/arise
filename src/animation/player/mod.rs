use bevy::prelude::*;
use assets::player::shadow::PlayerShadowAssets;
use engine::animation::AnimationComponent;
use engine::animation::states::{AnimationChangeEvent, AnimationState};
use engine::player::PlayerComponent;
use crate::animation::animation::AnimationsResource;

pub fn player_handle_animation_change(
    mut events: EventReader<AnimationChangeEvent>,
    animations_resource: Res<AnimationsResource>,
    player_assets: Res<PlayerShadowAssets>,
    mut query: Query<
        (
            Entity,
            &mut AnimationComponent,
            &mut Sprite,
            &mut AnimationState,
        ),
        With<PlayerComponent>,
    >,
) {
    for event in events.read() {
        log::info!(
            "Player animation change event for entity {:?} to state: {:?}",
            event.entity, event.state
        );

        let Ok((
                   _entity,
                   mut anim_component,
                   mut sprite,
                   mut current_anim_state,
               )) = query.get_mut(event.entity)
        else {
            log::warn!(
                "Failed to get player components for entity {:?} in AnimationChangeEvent.",
                event.entity
            );
            continue;
        };

        if *current_anim_state == event.state {
            log::info!("Animation state {:?} is already active and not delayed. Skipping.", event.state);
            continue;
        }

        if let Some(new_anim_data) = animations_resource.animations.get(&event.state) {
            log::info!(
                "Changing animation for entity {:?} from {:?} to {:?}",
                event.entity, *current_anim_state , event.state
            );

            *current_anim_state = event.state;
            anim_component.timer.pause();


            anim_component.timer =
                Timer::from_seconds(new_anim_data.frame_duration, new_anim_data.mode.into());
            anim_component.direction = new_anim_data.direction;
            anim_component.mode = new_anim_data.mode;

            let (image, layout) = match event.state {
                AnimationState::Idle => {
                    (player_assets.idle_image.clone(), player_assets.idle_layout.clone())
                }
                AnimationState::Running => {
                    (player_assets.run_image.clone(), player_assets.run_layout.clone())
                }
            };

            *sprite = Sprite::from_atlas_image(
                image,
                TextureAtlas::from(layout),
            );

            log::info!(
                "Successfully set entity {:?} to {:?} animation.",
                event.entity, event.state
            );
        } else {
            log::warn!(
                "Animation data not found in AnimationsResource for state: {:?} (entity: {:?}). Player animation unchanged.",
                event.state, event.entity
            );
        }


        anim_component.timer.reset();
        anim_component.timer.unpause();
        log::info!("Animation started.");
    }
}