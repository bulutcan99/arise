use crate::core::domain::component::base::Velocity;
use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Query, Res, Time, Transform};

pub fn models_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut moveable_query: Query<(&mut Transform, &mut Velocity)>,
    time: Res<Time>,
) {
    let mut combinations = moveable_query.iter_combinations_mut();
    while let Some([(mut transform_a, mut moveable_a), (mut transform_b, mut moveable_b)]) =
        combinations.fetch_next()
    {}
}
