use bevy::hierarchy::{ChildBuild, ChildBuilder};
use engine::abilities::shadow_monarch::{
    MonarchFormBundle, ShadowDashBundle, ShadowSummonBundle,
};
use engine::abilities::{
    self, AbilitiesResource, SlotOneAbilityType, SlotThreeAbilityType,
    SlotTwoAbilityType,
};

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
