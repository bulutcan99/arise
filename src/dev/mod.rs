use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::text::FontSmoothing;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use engine::states::AppStates;

/// `DevelopmentPlugin` is a Bevy plugin that provides runtime diagnostics
/// and developer tools such as FPS overlay, world inspector, and frame time logging.
///
/// This plugin includes:
/// - `FrameTimeDiagnosticsPlugin`: Logs frame timing metrics
/// - `LogDiagnosticsPlugin`: Logs diagnostic values to the console
/// - `WorldInspectorPlugin`: Enables real-time ECS inspection
/// - `FpsOverlayPlugin`: Shows an on-screen overlay with FPS metrics and styling options
///
/// It also includes keybinds for toggling and customizing the overlay:
/// - Press `1` to toggle between red and green text color
/// - Press `2` to decrease the overlay font size
/// - Press `3` to increase the overlay font size
/// - Press `4` to toggle overlay visibility
///
/// Example usage:
/// ```no_run
/// App::new()
///     .add_plugins(DefaultPlugins)
///     .add_plugin(DevelopmentPlugin);
/// ```
pub struct DevelopmentPlugin;

impl Plugin for DevelopmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
            WorldInspectorPlugin::new(),
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    text_config: TextFont {
                        // Here we define size of our overlay
                        font_size: 30.0,
                        // If we want, we can use a custom font
                        font: default(),
                        // We could also disable font smoothing,
                        font_smoothing: FontSmoothing::default(),
                        ..default()
                    },
                    // We can also change color of the overlay
                    text_color: OverlayColor::Green.color(),
                    ..Default::default()
                },
            },
        ))
            .add_systems(Startup, setup)
            .add_systems(Update, customize_config)
            .add_systems(Update, log_current_state);
    }
}

fn setup(mut commands: Commands) {
    // Instruction text

    commands.spawn((
        Text::new(concat!(
        "Press 1 to toggle the overlay color.\n",
        "Press 2 to decrease the overlay size.\n",
        "Press 3 to increase the overlay size.\n",
        "Press 4 to toggle the overlay visibility."
        )),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.),
            left: Val::Px(12.),
            ..default()
        },
    ));
    commands.insert_resource(LogTimer(Timer::from_seconds(10.0, TimerMode::Repeating)));
}

enum OverlayColor {
    Red,
    Green,
}

impl OverlayColor {
    pub fn color(&self) -> Color {
        match self {
            OverlayColor::Red => Color::srgb(1.0, 0.0, 0.0),
            OverlayColor::Green => Color::srgb(0.0, 1.0, 0.0),
        }
    }
}

fn customize_config(
    input: Res<ButtonInput<KeyCode>>,
    mut overlay: ResMut<FpsOverlayConfig>,
) {
    if input.just_pressed(KeyCode::Digit1) {
        // Changing resource will affect overlay
        if overlay.text_color == OverlayColor::Green.color() {
            overlay.text_color = OverlayColor::Red.color();
        } else {
            overlay.text_color = OverlayColor::Green.color();
        }
    }
    if input.just_pressed(KeyCode::Digit2) {
        overlay.text_config.font_size -= 2.0;
    }
    if input.just_pressed(KeyCode::Digit3) {
        overlay.text_config.font_size += 2.0;
    }
    if input.just_pressed(KeyCode::Digit4) {
        overlay.enabled = !overlay.enabled;
    }
}

#[derive(Resource)]
struct LogTimer(Timer);

fn log_current_state(
    time: Res<Time>,
    mut timer: ResMut<LogTimer>,
    state: Res<State<AppStates>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        log::info!("Current State: {:?}", state.get());
    }
}