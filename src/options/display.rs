use bevy::window::{MonitorSelection, Window, WindowMode, WindowResolution};

pub struct DisplayConfig {
    width: f32,
    height: f32,
    fullscreen: bool,
}

impl DisplayConfig {
    pub fn new() -> Self {
        Self {
            width: 1280.0,
            height: 1024.0,
            fullscreen: false,
        }
    }
}

impl From<DisplayConfig> for Window {
    fn from(value: DisplayConfig) -> Self {
        Window {
            ..Default::default()
        }
    }
}
