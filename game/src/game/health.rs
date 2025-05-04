use std::time::Duration;

use bevy::prelude::*;

/// Tracks health for an entity
#[derive(Component, Default)]
pub struct HealthComponent {
    /// Current health value
    health: u32,
    /// Maximum health value
    max_health: u32,
}

impl HealthComponent {
    pub fn new(health: u32, max_health: u32) -> Self {
        Self { health, max_health }
    }
}

#[derive(Component)]
pub struct HealthRegainComponent {
    /// Delay after last damage before regeneration starts.
    pub delay_timer: Timer,
    /// Interval between each health unit regenerated.
    pub interval_timer: Timer,
    /// How much health to regenerate each tick (e.g., 1 HP).
    pub amount_per_tick: u32,
}

impl Default for HealthRegainComponent {
    fn default() -> Self {
        Self {
            delay_timer: Timer::from_seconds(5.0, TimerMode::Once),
            interval_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            amount_per_tick: 1,
        }
    }
}
