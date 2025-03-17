use crate::core::domain::component::ai::Enemy;
use crate::core::domain::component::movement::Direction;
use crate::core::domain::entity::entity::Ai;
use crate::core::domain::system::ai::common::AI_ENEMY_SPEED;
use bevy::prelude::{Query, Res, Transform, With};
use bevy::time::Time;

//TODO: duvara carptiklarinda carptiklari aci ile sekmeleri
pub fn ai_enemy_movement(
    mut enemy_query: Query<(&mut Transform, &Direction), (With<Ai>, With<Enemy>)>,
    time: Res<Time>,
) {
    for (mut transform, direction) in enemy_query.iter_mut() {
        transform.translation += time.delta_secs() + direction.0 * AI_ENEMY_SPEED;
    }
}
