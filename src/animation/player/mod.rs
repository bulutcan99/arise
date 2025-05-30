use bevy::prelude::*;
use assets::player::shadow::PlayerShadowAssets;
use engine::animation::AnimationComponent;
use engine::animation::states::{AnimationChangeEvent, AnimationState};
use engine::player::PlayerComponent;
/*
pub fn player_handle_animation_change(
    mut events: EventReader<AnimationChangeEvent>,
    player_assets: Res<PlayerShadowAssets>,
    mut query: Query<(
        Entity,
        &mut AnimationComponent,
        &mut Sprite,
        &mut AnimationState,
    ), With<PlayerComponent>>,
) {
    for event in events.read() {
        log::info!("Player animation event triggered");
        let Ok((_entity, anim, mut sprite, mut animation_state)) = query.get_mut(event.entity) else {
            continue;
        };

        if *animation_state == event.state {
            continue;
        }

        match event.state {
            PlayerAnimationState::Idle => {
                *animation_state = PlayerAnimationState::Idle;
                // TODO: **** Simdilik sadece sprite degisimi yapalim
                // Sonrasinda anim componenti her animation icin okunaiblir hale getirip
                // degistigi zaman ordaki resource'u alip ordan frame saniyesini vs degisicez
                *sprite = Sprite::from_atlas_image(
                    player_assets.idle_image.clone(),
                    TextureAtlas::from(player_assets.idle_layout.clone()),
                )
            }
            PlayerAnimationState::Running => {
                *animation_state = PlayerAnimationState::Running;
                // TODO: **** Simdilik sadece sprite degisimi yapalim
                // Sonrasinda anim componenti her animation icin okunaiblir hale getirip
                // degistigi zaman ordaki resource'u alip ordan frame saniyesini vs degisicez
                *sprite = Sprite::from_atlas_image(
                    player_assets.run_image.clone(),
                    TextureAtlas::from(player_assets.run_layout.clone()),
                )
            }
            _ => {
                // TODO: Attack and other stuffs will add here
                todo!("Will implement")
            }
        }
    }
}

 */