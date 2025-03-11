use bevy::math::Vec2;
use bevy::prelude::Component;

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);
