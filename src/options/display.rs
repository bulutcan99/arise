use bevy::window::{MonitorSelection, Window, WindowMode, WindowResolution};
use crate::consts::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct DisplayConfig {
    width: f32,
    height: f32,
    fullscreen: bool,
}

impl DisplayConfig {
    pub fn new() -> Self {
        Self {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
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
