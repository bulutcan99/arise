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
use engine::character::{Character, CharacterType};
use engine::health::{HealthComponent, HealthRegainComponent};
use engine::input::{InputsResource, PlayerAction};
use engine::player::{
    PlayerBundle, PlayerData, PlayerIDComponent, PlayerVelocityComponent,
    PlayersResource,
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

fn prepare_player_character_data<'a>(
    players_res: &mut ResMut<PlayersResource>, /* Geçici olarak karakter seçimi burada */
    characters_res: &'a Res<CharactersResource>,
) -> Result<&'a Character, String> {
    // TODO*: sonrasinda bu kisim char selectiona tasinacak
    // Bu kısım normalde daha karmaşık bir karakter seçimi mekanizmasından gelebilir.
    // Şimdilik orijinal mantığı koruyoruz.
    if players_res.player_data.is_none() {
        players_res.player_data = Some(PlayerData {
            character: CharacterType::ShadowMonarch,
        });
    }

    let player_data = players_res
        .player_data
        .as_ref()
        .ok_or_else(|| "Player data missing after explicit set".to_string())?;
    characters_res
        .characters
        .get(&player_data.character)
        .ok_or_else(|| "Character definition not found".to_string())
}

fn add_visual_and_animation_components(
    entity_commands: &mut EntityCommands,
    char_data: &Character,
    game_parameters: &Res<GameResource>,
    player_assets: &Res<PlayerShadowAssets>,
    animations_res: &Res<AnimationsResource>,
    initial_animation_state: AnimationState,
) {
    let Some(animation_data) =
        animations_res.animations.get(&initial_animation_state)
    else {
        panic!(
            "Animation data for {:?} missing",
            initial_animation_state
        );
    };

    entity_commands.insert((
        Name::new(format!(
            "Player - {:?}",
            char_data.character_type
        )),
        Transform {
            translation: Vec3::ZERO,
            scale: Vec3::new(
                game_parameters.sprite_scale,
                game_parameters.sprite_scale,
                1.0,
            ),
            ..Default::default()
        },
        Sprite::from_atlas_image(
            player_assets.idle_image.clone(),
            TextureAtlas::from(player_assets.idle_layout.clone()),
        ),
        AnimationComponent::from(animation_data),
        initial_animation_state,
    ));
}

fn add_physics_components(
    entity_commands: &mut EntityCommands,
    char_data: &Character,
    game_parameters: &Res<GameResource>,
) {
    let collider_size_hx =
        char_data.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
    let collider_size_hy =
        char_data.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

    entity_commands.insert((PlayerVelocityComponent(0.0, 0.0)));
}

fn add_input_components(
    entity_commands: &mut EntityCommands,
    input_res: &Res<InputsResource>,
) {
    entity_commands.insert(InputManagerBundle::<PlayerAction> {
        action_state: ActionState::default(),
        input_map: input_res.player_keyboard.clone(),
    });
}

fn add_gameplay_components(
    entity_commands: &mut EntityCommands,
    char_data: &Character,
) {
    entity_commands.insert((
        HealthComponent::from(char_data),
        HealthRegainComponent::default(),
    ));
}

fn add_util_componenets(entity_commands: &mut EntityCommands) {
    entity_commands.insert(GameCleanup);
}

fn spawn_player_abilities(
    parent: &mut ChildBuilder,
    abilities_res: &Res<AbilitiesResource>,
    char_data: &Character,
) {
    parent.spawn_slot_1_ability(
        abilities_res,
        char_data.slot_1_ability.as_ref(),
    );
    parent.spawn_slot_2_ability(
        abilities_res,
        char_data.slot_2_ability.as_ref(),
    );
    parent.spawn_slot_3_ability(
        abilities_res,
        char_data.slot_3_ability.as_ref(),
    );
}

pub fn spawn_player_system(
    mut commands: Commands,
    characters_res: Res<CharactersResource>,
    game_parameters: Res<GameResource>,
    player_assets: Res<PlayerShadowAssets>,
    mut players_res: ResMut<PlayersResource>,
    animations_res: Res<AnimationsResource>,
    input_res: Res<InputsResource>,
    abilities_res: Res<AbilitiesResource>,
) {
    let char_data = match prepare_player_character_data(
        &mut players_res,
        &characters_res,
    ) {
        Ok(data) => data,
        Err(e) => {
            error!(
                "Failed to prepare player character data: {}",
                e
            );
            return;
        },
    };

    let player_bundle =
        PlayerBundle::from(char_data).with_id(PlayerIDComponent::One);
    let initial_animation_state = AnimationState::Run;

    let mut player_entity_commands = commands.spawn_empty();

    player_entity_commands.insert(player_bundle);

    add_visual_and_animation_components(
        &mut player_entity_commands,
        char_data,
        &game_parameters,
        &player_assets,
        &animations_res,
        initial_animation_state,
    );

    add_physics_components(
        &mut player_entity_commands,
        char_data,
        &game_parameters,
    );

    add_input_components(&mut player_entity_commands, &input_res);
    add_gameplay_components(&mut player_entity_commands, char_data);
    add_util_componenets(&mut player_entity_commands);

    player_entity_commands.with_children(|parent| {
        spawn_player_abilities(parent, &abilities_res, char_data);
    });

    info!(
        "Player {:?} spawned",
        char_data.character_type
    );
}
