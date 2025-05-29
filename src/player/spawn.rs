use bevy::hierarchy::{ChildBuild, ChildBuilder};
use bevy::log;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use assets::player::shadow::PlayerShadowAssets;
use engine::abilities::shadow_monarch::{
    MonarchFormBundle, ShadowDashBundle, ShadowSummonBundle,
};
use engine::abilities::{
    self, AbilitiesResource, SlotOneAbilityType, SlotThreeAbilityType,
    SlotTwoAbilityType,
};
use engine::input::{InputsResource, PlayerAction, PlayerInput};
use engine::player::{PlayerBundle, PlayerIDComponent, PlayersResource};
use engine::health::{HealthComponent, HealthRegainComponent};
use engine::states::GameCleanup;
use crate::animation::{AnimationComponent, AnimationDirection};
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
    players_resource: Res<PlayersResource>,
    abilities_res: Res<AbilitiesResource>,
    // TODO*: menu olacak ancak char selection simdilik olmayacak
) {
    // TODO: simdilik tek kullanici varmis gibi dusunelim karakter secme ekraninda doldurulcak resleri burda dolduralim simdilik
    // TODO: let mut is_multiplayer = players_resource.player_data.get(1).is_some();

    // TODO: sonrasinda asagidaki kisim iter olacak sekilde duzenlenecek
    let maybe_player_one = players_resource.player_data.get(0).unwrap();
    let Some(player_one) = maybe_player_one else {
        return;
    };

    let char = characters.characters.get(&player_one.character).unwrap();
    let collider_size_hx = char.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
    let collider_size_hy = char.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

    let player_bundle = PlayerBundle::from(char).with_id(PlayerIDComponent::One);
    let mut player_entity = commands.spawn_empty();
    player_entity.insert(player_bundle)
        .insert((
            Sprite::from_atlas_image(
                player_assets.run_image.clone(),
                TextureAtlas::from(player_assets.run_layout.clone()),
            ),
            Transform::from_xyz(0., 0., 0.),
            AnimationComponent {
                timer: Timer::from_seconds(
                    0.1, TimerMode::Repeating),
                direction: AnimationDirection::Forward,
            },
        ))
        .insert(RigidBody::Dynamic).insert(
        Transform {
            translation: Vec3::ZERO,

            scale: Vec3::new(
                game_parameters.sprite_scale,
                game_parameters.sprite_scale,
                1.0,
            ),
            rotation: Default::default(),
        }).insert(Collider::cuboid(collider_size_hx, collider_size_hy))
        .insert(Velocity::default())
        .insert(Restitution::new(1.0))
        .insert(ColliderMassProperties::Density(char.collider_density))
        .insert(HealthComponent::from(char))
        .insert(HealthRegainComponent::default())
        .insert(GameCleanup)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ExternalImpulse::default())
        .insert(Name::new("Player"))
        .with_children(|parent| {
            parent.spawn_slot_1_ability(&abilities_res, char.slot_1_ability.as_ref());
            parent.spawn_slot_2_ability(&abilities_res, char.slot_2_ability.as_ref());
            parent.spawn_slot_3_ability(&abilities_res, char.slot_3_ability.as_ref());
        });
}