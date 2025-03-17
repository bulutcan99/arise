use bevy::math::Vec3;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Direction(pub Vec3);

#[derive(Component)]
pub struct Velocity(pub Vec3);
