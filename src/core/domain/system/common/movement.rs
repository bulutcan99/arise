use crate::core::domain::component::base::Velocity;
use crate::core::domain::system::common::common::BALL_SIZE;
use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Query, Res, Time, Transform};

pub fn models_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut moveable_query: Query<(&mut Transform, &mut Velocity)>,
    time: Res<Time>,
) {
    let mut combinations = moveable_query.iter_combinations_mut();
    while let Some([(mut transform_a, mut velocity_a), (mut transform_b, mut velocity_b)]) =
        combinations.fetch_next()
    {
        transform_a.translation += velocity_a.0 * time.delta_secs();
        transform_b.translation += velocity_b.0 * time.delta_secs();
        let collision = (transform_a.translation.x - transform_b.translation.x).abs() < BALL_SIZE
            && (transform_a.translation.y - transform_b.translation.y).abs() < BALL_SIZE
            && (transform_a.translation.z - transform_b.translation.z).abs() < BALL_SIZE;

        if collision {
            velocity_a.0 = -velocity_a.0;
            velocity_b.0 = -velocity_b.0;
        }
    }
}
