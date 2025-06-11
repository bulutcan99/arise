mod systems;

use bevy::app::App;
use bevy::math::Vec3;
use bevy::prelude::{in_state, Commands, IntoSystemConfigs, Name, OnEnter, Plugin, Res, ResMut, Transform, Update};
use bevy::sprite::Sprite;
use assets::player::shadow::PlayerShadowAssets;
use assets::weapon::WeaponAssets;
use engine::abilities::AbilitiesResource;
use engine::input::InputsResource;
use engine::player::PlayersResource;
use engine::states::app::AppStates;
use engine::weapon::WeaponComponent;
use crate::animation::animation::AnimationsResource;
use crate::game::resources::GameResource;
use crate::player::character::CharactersResource;
use crate::weapon::systems::transform::update_weapon_transform;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_weapon_transform).run_if(in_state(AppStates::InGame))
        );
    }
}

pub fn spawn_weapon_system(
    mut commands: Commands,
    characters_res: Res<CharactersResource>,
    game_parameters: Res<GameResource>,
    weapon_assets: Res<WeaponAssets>,
) {
    let mut weapon_entity_commands = commands.spawn_empty();
    weapon_entity_commands.insert(WeaponComponent);
    weapon_entity_commands.insert(
        (
            Name::new("Ice Staff"),
            Transform {
                translation: Vec3::ZERO,
                scale: Vec3::new(
                    0.8,
                    0.8,
                    1.0,
                ),
                ..Default::default()
            },
            Sprite {
                image: weapon_assets.staff_image.clone(),
                ..Default::default()
            }
        )
    );
    log::info!(
        "Weapon spawned!"
    );
}