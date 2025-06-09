use bevy::app::App;
use bevy::asset::ron::de::from_bytes;
use bevy::prelude::*;
use engine::abilities::{
    AbilitiesResource, AbilityDescriptionsResource, ActivateAbilityEvent,
};
use engine::input::PlayerAction;
use engine::player::PlayersResource;
use engine::states::AppStates;
use leafwing_input_manager::plugin::InputManagerPlugin;
use systems::movement::movement_system;

use crate::player::character::CharactersResource;
use crate::player::systems::attack::light_attack_system;

pub mod character;
pub mod spawn;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());
        app.add_event::<ActivateAbilityEvent>();

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
            (movement_system, light_attack_system)
                .run_if(in_state(AppStates::InGame)),
        );
    }
}
