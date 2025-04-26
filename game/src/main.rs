use bevy::prelude::*;

mod assets;
mod camera;
mod consts;
mod game;
mod map;
pub mod player;
mod state;
mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Roguelike Game".to_string(),
                        resolution: (
                            consts::WINDOW_WIDTH as f32 * 10.0,
                            consts::WINDOW_HEIGHT as f32 * 10.0,
                        )
                            .into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .insert_resource(ClearColor(Color::srgb_u8(1, 50, 45)))
        .run();
}
