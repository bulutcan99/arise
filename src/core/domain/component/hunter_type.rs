use bevy::prelude::Component;

//TODO:
//Her classin kendine ait ozel silah ve
//skilleri bulunacak, necro mesela oluleri diriltme gibi

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HunterKind {
    Fighter,
    Mage,
    Necro,
    Ranger,
}

#[derive(Component)]
pub struct Fighter;

#[derive(Component)]
pub struct Mage;

#[derive(Component)]
pub struct Necro;

#[derive(Component)]
pub struct Assassin;
