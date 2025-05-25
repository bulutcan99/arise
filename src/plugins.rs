use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};

use crate::{ animation, camera, development, game::{self, counters}, settings::PHYSICS_PIXELS_PER_METER, states};

pub struct ArisePlugins;

impl PluginGroup for ArisePlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        #[allow(unused_mut)] // Allow because we might add more platform-specific features
        let mut res = PluginGroupBuilder::start::<Self>()
            .add(
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(PHYSICS_PIXELS_PER_METER)
                    .in_fixed_schedule(),
            )
            .add(states::StatesPlugin)
            .add(development::DevelopmentPlugin)
            .add(animation::SpriteAnimationPlugin)
            .add(game::GameResourcePlugin)
            .add(counters::CounterPlugin)
            .add(camera::CameraPlugin)

    }
}
