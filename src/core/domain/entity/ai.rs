use bevy::prelude::Component;

#[derive(Component)]
pub struct Enemy(pub u32);

#[derive(Component)]
pub struct Friendly(pub u32);
