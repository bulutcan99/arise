use bevy::app::App;
use bevy::asset::ron::de::from_bytes;
use bevy::prelude::*;
use engine::abilities::{
    AbilitiesResource, AbilityDescriptionsResource,
};
use engine::input::PlayerAction;
use engine::player::PlayersResource;
use engine::states::app::AppStates;
use leafwing_input_manager::plugin::InputManagerPlugin;
use engine::events::action::{DashEvent, HeavyAttackEvent, LightAttackEvent, MoveEvent, UseSkillEvent};
use crate::player::character::CharactersResource;
use crate::player::systems::input::player_input_router_system;
use crate::player::systems::movement::movement::movement_system;

pub mod character;
pub mod spawn;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
        app.add_event::<MoveEvent>();
        app.add_event::<DashEvent>();
        app.add_event::<LightAttackEvent>();
        app.add_event::<HeavyAttackEvent>();
        app.add_event::<UseSkillEvent>();


        app.insert_resource(
            from_bytes::<CharactersResource>(include_bytes!(
                "../../assets/data/characters.ron"
            ))
            .unwrap(),
        );

        app.insert_resource(
            from_bytes::<AbilitiesResource>(include_bytes!(
                "../../assets/data/abilities.ron"
            ))
            .unwrap(),
        );

        app.insert_resource(
            from_bytes::<AbilityDescriptionsResource>(include_bytes!(
                "../../assets/data/ability_descriptions.ron"
            ))
            .unwrap(),
        );

        app.insert_resource(PlayersResource::default());

        app.add_systems(
            Update,
            (player_input_router_system, movement_system)
                .run_if(in_state(AppStates::InGame)),
        );
    }
}
