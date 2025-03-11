use bevy::prelude::Component;

#[derive(Component)]
pub struct Health(pub u32);

#[derive(Component)]
pub struct Mana(pub u32);

#[derive(Component)]
pub struct Stamina(pub u32);

#[derive(Component)]
pub struct Level(pub u32);

#[derive(Component)]
pub struct Experience(pub u32);
