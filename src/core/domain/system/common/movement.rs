use crate::core::domain::component::ai::Enemy;
use crate::core::domain::component::movement::Direction;
use crate::core::domain::entity::entity::{Ai, Player};
use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Query, Res, Transform, With};

pub fn models_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut Transform, &mut Direction), With<Player>>,
    mut enemy_query: Query<(&mut Transform, &mut Direction), (With<Ai>, With<Enemy>)>,
) {
    todo!()
}
