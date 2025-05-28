use std::time::Duration;

use bevy::prelude::*;

use crate::character::Character;

/// Component that tracks current and maximum health for an entity.
#[derive(Component)]
pub struct HealthComponent {
    /// Current health value. Always <= `max_health`.
    pub current: u32,
    /// Maximum health value. Entities cannot exceed this.
    pub max: u32,
}

impl Default for HealthComponent {
    fn default() -> Self {
        Self {
            current: 100,
            max: 100,
        }
    }
}

impl HealthComponent {
    /// Creates a new HealthComponent with the given maximum health.
    /// The entity starts at full health.
    pub fn new(max_health: u32) -> Self {
        Self {
            current: max_health,
            max: max_health,
        }
    }

    /// Restores health to maximum.
    pub fn full_heal(&mut self) {
        self.current = self.max;
    }

    /// Returns `true` if the entity's current health is zero.
    pub fn is_dead(&self) -> bool {
        self.current == 0
    }

    /// Applies damage, subtracting from current health.
    /// Saturates at zero (no underflow).
    pub fn take_damage(&mut self, amount: u32) {
        self.current = self.current.saturating_sub(amount);
    }

    /// Heals the entity by the specified amount, without exceeding max.
    pub fn heal(&mut self, amount: u32) {
        self.current = (self.current + amount).min(self.max);
    }

    /// Returns the fraction of health remaining (0.0 to 1.0).
    pub fn health_percentage(&self) -> f32 {
        if self.max > 0 {
            self.current as f32 / self.max as f32
        } else {
            0.0
        }
    }

    /// Permanently increases the maximum health by `value`.
    /// Does not change current health.
    pub fn increase_max(&mut self, value: u32) {
        self.max = self.max.saturating_add(value);
    }
}

impl From<&Character> for HealthComponent {
    fn from(value: &Character) -> Self {
        HealthComponent::new(value.health)
    }
}

/// Component that handles passive health regeneration when not taking damage.
#[derive(Component)]
pub struct HealthRegainComponent {
    /// Delay (in seconds) after last damage before regeneration starts.
    pub delay_timer: Timer,
    /// Interval (in seconds) between each regeneration tick.
    pub interval_timer: Timer,
    /// Amount of health to regenerate each tick.
    pub amount_per_tick: u32,
}

impl Default for HealthRegainComponent {
    fn default() -> Self {
        Self {
            // Wait 5s after last damage before regen begins
            delay_timer: Timer::from_seconds(5.0, TimerMode::Once),
            // Once started, regen occurs every 0.5s
            interval_timer: Timer::from_seconds(1., TimerMode::Repeating),
            amount_per_tick: 1,
        }
    }
}

impl HealthRegainComponent {
    /// Resets both timers, called when entity takes damage.
    pub fn reset(&mut self) {
        self.delay_timer.reset();
        self.interval_timer.reset();
    }

    /// Updates regeneration logic.
    /// - `delta`: Time since last frame.
    /// - `health`: Mutable reference to the entity's HealthComponent.
    pub fn update(&mut self, delta: Duration, health: &mut HealthComponent) {
        self.delay_timer.tick(delta);
        if !self.delay_timer.finished() {
            return;
        }

        self.interval_timer.tick(delta);
        if self.interval_timer.just_finished() {
            health.heal(self.amount_per_tick);
        }
    }
}

