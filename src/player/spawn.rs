use assets::player::shadow::PlayerShadowAssets;
use bevy::hierarchy::{ChildBuild, ChildBuilder};
use bevy::prelude::*;
use engine::abilities::shadow_monarch::{
    MonarchFormBundle, ShadowDashBundle, ShadowSummonBundle,
};
use engine::abilities::{
    AbilitiesResource, SlotOneAbilityType, SlotThreeAbilityType,
    SlotTwoAbilityType,
};
use engine::animation::AnimationComponent;
use engine::animation::states::AnimationState;
use engine::character::CharacterType;
use engine::health::{HealthComponent, HealthRegainComponent};
use engine::input::PlayerInput;
use engine::player::{
    PlayerBundle, PlayerData, PlayerIDComponent, PlayersResource,
};
use engine::states::GameCleanup;

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
                }
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
                }
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
                }
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
    mut players_resource: ResMut<PlayersResource>,
    animations_resource: ResMut<AnimationsResource>,
    abilities_res: Res<AbilitiesResource>,
    // TODO*: menu ve char selection simdilik olmayacak calisir hale getirince eklenecek
) {
    // TODO*: sonrasinda bu kisim char selectiona tasinacak
    players_resource.player_data = Some(PlayerData {
        input: PlayerInput::Keyboard,
        character: CharacterType::ShadowMonarch,
    });

    let Some(player_one) = players_resource.player_data.as_ref() else {
        error!("Player data missing");
        return;
    };

    let char = characters.characters.get(&player_one.character).unwrap();
    let player_bundle =
        PlayerBundle::from(char).with_id(PlayerIDComponent::One);
    let animation_state = AnimationState::Idle;
    let Some(animation_data) = animations_resource.animations.get(&animation_state) else {
        panic!("Animation data missing")
    };

    let mut player_entity = commands.spawn_empty();
    player_entity
        .insert(player_bundle)
        .insert(
            Sprite::from_atlas_image(
                player_assets.run_image.clone(),
                TextureAtlas::from(player_assets.run_layout.clone()),
            ))
        .insert(AnimationComponent::from(animation_data))
        .insert(animation_state)
        .insert(Transform::from_xyz(0., 0., 0.))
        .insert(HealthComponent::from(char))
        .insert(HealthRegainComponent::default())
        .insert(GameCleanup)
        .insert(Name::new("Shadow"))
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
