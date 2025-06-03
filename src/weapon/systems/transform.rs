use bevy::prelude::*;
use engine::player::{
    PlayerComponent, PlayerMobilityComponent, PlayerVelocityComponent,
};
use engine::weapon::WeaponComponent;

pub fn update_weapon_transform(
    mut weapon_query: Query<
        &mut Transform,
        (
            With<WeaponComponent>,
            Without<PlayerComponent>,
        ),
    >,
    player_query: Query<&Transform, With<PlayerComponent>>,
) {
    if player_query.is_empty() || weapon_query.is_empty() {
        warn!("Weapon or player is empty");
        return;
    }

    let player_transform = player_query.single();
    let mut weapon_transform = weapon_query.single_mut(); // .single_mut() ile mutable referans alıyoruz

    weapon_transform.translation = player_transform.translation;
}

