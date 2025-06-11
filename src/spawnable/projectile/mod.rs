pub mod systems;

use bevy::prelude::*;
use assets::spawnable::projectile::ProjectileAssets;
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
            (update_projectile_transform).run_if(in_state(AppStates::InGame))
        );
    }
}

pub fn spawn_projectile_system(
    mut commands: Commands,
    characters_res: Res<CharactersResource>,
    game_parameters: Res<GameResource>,
    projectile_assets: Res<ProjectileAssets>,
) {
    let mut projectile_entity_commands = commands.spawn_empty();
    projectile_entity_commands.insert(ProjectileComponent);
    projectile_entity_commands.insert(
        (
            Name::new("Bullet"),
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
                image: projectile_assets.ally_bullet_image.clone(),
                ..Default::default()
            }
        )
    );
    log::info!(
        "Projectile spawned!"
    );
}