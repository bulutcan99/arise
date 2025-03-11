use crate::core::domain::component::attribute::{Dexterity, Intelligence, Strength};
use crate::core::domain::component::class::Warrior;
use crate::core::domain::component::combat::{AttackPower, Defense};
use crate::core::domain::component::movement::{Position, Velocity};
use crate::core::domain::component::stat::{Experience, Health, Level, Mana, Stamina};
use bevy::asset::{AssetServer, Assets};
use bevy::prelude::{Camera2d, Commands, Res, ResMut, TextureAtlasLayout, Vec2};

pub fn spawn_user(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Warrior,
        Strength(1),
        Dexterity(1),
        Intelligence(1),
        AttackPower(1),
        Defense(1),
        Position(Vec2::new(0., 0.)),
        Velocity(Vec2::new(0., 0.)),
        Health(100),
        Mana(50),
        Stamina(10),
        Level(1),
        Experience(0),
    ));
}
