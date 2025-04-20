use bevy::prelude::Component;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonsterKind {
    Mage,
    Tank,
    Healer,
    Range,
}

#[derive(Component)]
pub struct Mage;

#[derive(Component)]
pub struct Tank;

#[derive(Component)]
pub struct Healer;

#[derive(Component)]
pub struct Range;
