use bevy::window::{Window, WindowMode};

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
            titlebar_show_title: "ARISE".to_string(),
            resolution: (value.width, value.height).into(),
            resizable: true,
            mode: if value.fullscreen {
                return WindowMode::BorderlessFullscreen(
                    bevy::window::MonitorSelection::Primary,
                );
            } else {
                return WindowMode::Windowed;
            },
            ..Default::default()
        }
    }
}
