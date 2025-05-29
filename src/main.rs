use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;
use engine::states::AppStates;
use options::display::DisplayConfig;
use crate::consts::SIMULATION_SCALE_FACTOR;
use crate::game::counters;
use crate::options::PHYSICS_PIXELS_PER_METER;

mod animation;
mod camera;
mod consts;
mod dev;
mod game;
mod options;
mod player;
mod states;
mod ui;
mod combat;
mod run;
mod scanner;
mod spawnable;

// TODO: **** Player'i ortala spawn olurken su an gorunmuyor ****
fn main() {
    let display_config = set_display_config();
    let mut app = build_app(
        default_plugins(display_config),
        custom_plugins(),
    );

    app.run();
}

// TODO: Player will set display!
fn set_display_config() -> DisplayConfig {
    DisplayConfig::new()
}

fn default_plugins(display: DisplayConfig) -> PluginGroupBuilder {
    DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window::from(display)),
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
}

fn custom_plugins() -> PluginGroupBuilder {
    #[allow(unused_mut)]
    let mut res = ArisePlugins.build();
    res
}

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
            .add(dev::DevelopmentPlugin)
            .add(states::StatesPlugin)
            .add(animation::SpriteAnimationPlugin)
            .add(game::GameResourcePlugin)
            .add(camera::CameraPlugin)
            .add(combat::CombatPlugin)
            .add(player::PlayerPlugin);

        res
    }
}

/// Make the runnable platform-specific app. `base_plugins` describes "external dependencies"
/// outside the scope of the game itself. These typically come from `bevy::MinimalPlugins` or
/// `bevy::DefaultPlugins`. `game_plugins` comes from from `ThetawaveGamePlugins`.
fn build_app<P1: PluginGroup, P2: PluginGroup>(base_plugins: P1, game_plugins: P2) -> App {
    // Should everything beside
    let mut app = App::new();
    app.add_plugins(base_plugins);
    app.init_state::<AppStates>(); // start game in the main menu state
    app.add_plugins(game_plugins);
    app.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight::default());

    app.add_systems(
        OnEnter(AppStates::Game),
        setup_physics,
    );
    if cfg!(debug_assertions) && !cfg!(test) {
        app.add_plugins(RapierDebugRenderPlugin::default());
    }
    app
}

fn setup_physics(mut commands: Commands, mut rapier_timestamp: ResMut<TimestepMode>) {
    let rapier_config = RapierConfiguration::new(SIMULATION_SCALE_FACTOR);
    commands.spawn(rapier_config);

    *rapier_timestamp = TimestepMode::default();
}
