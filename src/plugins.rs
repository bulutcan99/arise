use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy_rapier2d::plugin::{NoUserData, RapierPhysicsPlugin};

use crate::game::{self, counters};
use crate::options::PHYSICS_PIXELS_PER_METER;
use crate::{player, states, dev, animation, camera, combat};

pub struct ArisePlugins;

impl PluginGroup for ArisePlugins {
    fn build(self) -> PluginGroupBuilder {
        #[allow(unused_mut)]
        // Allow because we might add more platform-specific features
        let mut res = PluginGroupBuilder::start::<Self>()
            .add(
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
                    PHYSICS_PIXELS_PER_METER,
                )
                    .in_fixed_schedule(),
            )
            .add(states::StatesPlugin)
            .add(dev::DevelopmentPlugin)
            .add(animation::SpriteAnimationPlugin)
            .add(game::GameResourcePlugin)
            .add(counters::CounterPlugin)
            .add(camera::CameraPlugin)
            .add(combat::CombatPlugin)
            .add(player::PlayerPlugin);

        res
    }
}
