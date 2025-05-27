use bevy::hierarchy::{ChildBuild, ChildBuilder};
use bevy::prelude::{Commands, Res};
use assets::player::shadow::PlayerShadowAssets;
use engine::abilities::shadow_monarch::{
    MonarchFormBundle, ShadowDashBundle, ShadowSummonBundle,
};
use engine::abilities::{
    self, AbilitiesResource, SlotOneAbilityType, SlotThreeAbilityType,
    SlotTwoAbilityType,
};
use engine::input::InputsResource;
use engine::player::PlayersResource;
use crate::game::resources::GameResource;
use crate::player::character::CharacterResource;

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
    characters: Res<CharacterResource>,
    game_parameters: Res<GameResource>,
    // TODO: sonrasi icin spawn olcak tipe gore asset eklenecek (bunun icin tum assetler loadlandiktan sonra
    // hepsini bir struct altinda ayri bir resource altinda toplayabiliriz
    player_assets: Res<PlayerShadowAssets>,
    players_resource: Res<PlayersResource>,
    inputs_res: Res<InputsResource>,
    abilities_res: Res<AbilitiesResource>,
){
    // TODO: simdilik tek kullanici varmis gibi dusunelim
    let mut is_multiplayer = players_resource.player_data.get(1).is_some();
    is_multiplayer = false;

    // TODO: sonrasinda asagidaki kisim iter olacak sekilde duzenlenecek
    let maybe_player_one = players_resource.player_data.get(0).unwrap();
    let Some(player_one) = maybe_player_one else{
        return;
    };

    let char = characters.characters.get(&player_one.character).unwrap();



}