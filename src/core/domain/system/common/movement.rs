use crate::core::domain::component::base::{Size, Velocity};
use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Query, Res, Time, Transform};

//TODO: toplara player bir anda yaklastiginda ai'lar buga giriyor
pub fn models_collision(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut moveable_query: Query<(&mut Transform, &mut Velocity, &Size)>,
    time: Res<Time>,
) {
    let mut combinations = moveable_query.iter_combinations_mut();
    while let Some(
        [(mut transform_a, mut velocity_a, size_a), (mut transform_b, mut velocity_b, size_b)],
    ) = combinations.fetch_next()
    {
        transform_a.translation += velocity_a.0 * time.delta_secs();
        transform_b.translation += velocity_b.0 * time.delta_secs();

        let half_size_a = size_a.0 / 2.0;
        let half_size_b = size_b.0 / 2.0;

        let collision = (transform_a.translation.x - transform_b.translation.x).abs()
            < (half_size_a.x + half_size_b.x)
            && (transform_a.translation.y - transform_b.translation.y).abs()
                < (half_size_a.y + half_size_b.y)
            && (transform_a.translation.z - transform_b.translation.z).abs()
                < (half_size_a.z + half_size_b.z);

        if collision {
            velocity_a.0 = -velocity_a.0;
            velocity_b.0 = -velocity_b.0;
        }
    }
}
