pub mod systems;

use assets::spawnable::projectile::ProjectileAssets;
use bevy::prelude::*;
use engine::spawnable::projectile::ProjectileComponent;
use engine::states::app::AppStates;

use crate::game::resources::GameResource;
use crate::player::character::CharactersResource;
use crate::spawnable::projectile::systems::transform::update_projectile_transform;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_projectile_transform).run_if(in_state(AppStates::InGame)),
        );
    }
}
