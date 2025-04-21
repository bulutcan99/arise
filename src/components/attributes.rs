use bevy::{ecs::bundle::Bundle, prelude::Component};

#[derive(Component, Debug, Clone)]
pub struct Attributes {
    pub str: u16,
    pub dex: u16,
    pub int: u16,
}
