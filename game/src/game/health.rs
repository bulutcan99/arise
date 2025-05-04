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
    pub fn new(max_health: u32) -> Self {
        Self {
            health: max_health.clone(),
            max_health,
        }
    }

    pub fn is_dead(&self) -> bool {
        if self.max_health <= 0 {
            return true;
        }
        false
    }

    pub fn take_damage(&mut self, taken_damage: u32) {
        self.health -= taken_damage
    }

    pub fn heal(&mut self, health: u32) {
        self.health = (self.health + health).min(self.max_health);
    }

    pub fn health_percentage(&self) -> f32 {
        if self.max_health > 0 {
            return self.health as f32 / self.max_health as f32;
        }
        0.0
    }

    pub fn increase_max_health(&mut self, value: u32) {
        self.max_health += value;
    }

    pub fn full_heal(&mut self) {
        self.health = self.max_health;
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
