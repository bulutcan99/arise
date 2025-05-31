use assets::player::shadow::PlayerShadowAssets;
use bevy::hierarchy::{ChildBuild, ChildBuilder};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use engine::abilities::shadow_monarch::{
    MonarchFormBundle, ShadowDashBundle, ShadowSummonBundle,
};
use engine::abilities::{
    AbilitiesResource, SlotOneAbilityType, SlotThreeAbilityType,
    SlotTwoAbilityType,
};
use engine::animation::states::AnimationState;
use engine::animation::AnimationComponent;
use engine::character::CharacterType;
use engine::health::{HealthComponent, HealthRegainComponent};
use engine::input::{InputsResource, PlayerAction};
use engine::player::{
    PlayerBundle, PlayerData, PlayerIDComponent, PlayersResource,
};
use engine::states::GameCleanup;
use leafwing_input_manager::prelude::ActionState;
use leafwing_input_manager::InputManagerBundle;

use crate::animation::animation::AnimationsResource;
use crate::game::resources::GameResource;
use crate::player::character::CharactersResource;

trait PlayerAbilityChildBuilderExt {
    fn spawn_slot_1_ability(
        &mut self,
        ability_res: &AbilitiesResource,
        ability_type: Option<&SlotOneAbilityType>,
    );

    fn spawn_slot_2_ability(
        &mut self,
        ability_res: &AbilitiesResource,
        ability_type: Option<&SlotTwoAbilityType>,
    );

    fn spawn_slot_3_ability(
        &mut self,
        ability_res: &AbilitiesResource,
        ability_type: Option<&SlotThreeAbilityType>,
    );
}

impl PlayerAbilityChildBuilderExt for ChildBuilder<'_> {
    fn spawn_slot_1_ability(
        &mut self,
        abilities_res: &AbilitiesResource,
        ability_type: Option<&SlotOneAbilityType>,
    ) {
        if let Some(ability_one) = ability_type {
            match ability_one {
                SlotOneAbilityType::ShadowSummon => {
                    self.spawn(ShadowSummonBundle::from(
                        &abilities_res.shadow_summon,
                    ))
                },
            };
        }
    }

    fn spawn_slot_2_ability(
        &mut self,
        abilities_res: &AbilitiesResource,
        ability_type: Option<&SlotTwoAbilityType>,
    ) {
        if let Some(ability_two) = ability_type {
            match ability_two {
                SlotTwoAbilityType::ShadowDash => {
                    self.spawn(ShadowDashBundle::from(
                        &abilities_res.shadow_dash,
                    ))
                },
            };
        }
    }

    fn spawn_slot_3_ability(
        &mut self,
        abilities_res: &AbilitiesResource,
        ability_type: Option<&SlotThreeAbilityType>,
    ) {
        if let Some(ability_three) = ability_type {
            match ability_three {
                SlotThreeAbilityType::MonarchForm => {
                    self.spawn(MonarchFormBundle::from(
                        &abilities_res.monarch_form,
                    ))
                },
            };
        }
    }
}

pub fn spawn_player_system(
    mut commands: Commands,
    characters: Res<CharactersResource>,
    game_parameters: Res<GameResource>,
    // TODO: sonrasi icin spawn olcak tipe gore asset eklenecek (bunun icin tum assetler loadlandiktan sonra
    // hepsini bir struct altinda ayri bir resource altinda toplayabiliriz
    player_assets: Res<PlayerShadowAssets>,
    mut players_res: ResMut<PlayersResource>,
    animations_res: ResMut<AnimationsResource>,
    input_res: Res<InputsResource>,
    abilities_res: Res<AbilitiesResource>,
    // TODO*: menu ve char selection simdilik olmayacak calisir hale getirince eklenecek
) {
    // TODO*: sonrasinda bu kisim char selectiona tasinacak
    players_res.player_data = Some(PlayerData {
        character: CharacterType::ShadowMonarch,
    });

    let Some(player_one) = players_res.player_data.as_ref() else {
        error!("Player data missing");
        return;
    };

    let char = characters.characters.get(&player_one.character).unwrap();
    // scale collider to align with the sprite
    let collider_size_hx =
        char.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
    let collider_size_hy =
        char.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

    let player_bundle =
        PlayerBundle::from(char).with_id(PlayerIDComponent::One);
    let animation_state = AnimationState::Idle;
    let Some(animation_data) = animations_res.animations.get(&animation_state) else {
        panic!("Animation data missing")
    };

    let mut player_entity = commands.spawn_empty();
    player_entity
        .insert(player_bundle)
        .insert(Name::new("Shadow"))
        .insert(Transform {
            translation: Vec3::ZERO,
            scale: Vec3::new(
                game_parameters.sprite_scale,
                game_parameters.sprite_scale,
                1.0,
            ),
            ..Default::default()
        })
        .insert(Sprite::from_atlas_image(
            player_assets.idle_image.clone(),
            TextureAtlas::from(player_assets.idle_layout.clone()),
        ))
        .insert(AnimationComponent::from(animation_data))
        .insert(animation_state)
        .insert(HealthComponent::from(char))
        .insert(HealthRegainComponent::default())
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map: input_res.player_keyboard.clone(),
        })
        .insert(Collider::cuboid(
            collider_size_hx,
            collider_size_hy,
        ))
        .insert(Velocity::default())
        .insert(Restitution::new(1.0))
        .insert(ColliderMassProperties::Density(
            char.collider_density,
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ExternalImpulse::default())
        .insert(GameCleanup)
        .with_children(|parent| {
            parent.spawn_slot_1_ability(
                &abilities_res,
                char.slot_1_ability.as_ref(),
            );
            parent.spawn_slot_2_ability(
                &abilities_res,
                char.slot_2_ability.as_ref(),
            );
            parent.spawn_slot_3_ability(
                &abilities_res,
                char.slot_3_ability.as_ref(),
            );
        });

    info!("Player spawned");
}
