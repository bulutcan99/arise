use bevy::{ecs::bundle::Bundle, prelude::Component};

#[derive(Component)]
pub struct Strength(pub u16);

#[derive(Component)]
pub struct Dexterity(pub u16);

#[derive(Component)]
pub struct Intelligence(pub u16);

#[derive(Bundle)]
pub struct AttributesBundle {
    pub str: Strength,
    pub dex: Dexterity,
    pub int: Intelligence,
}
