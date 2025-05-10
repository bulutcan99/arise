use bevy::prelude::*;
use serde::Deserialize;

/// Event triggered when an entity (e.g., an enemy) is defeated.
#[derive(Event)]
pub struct EnemyDefeatedEvent {
    pub entity: Entity,
}

/// Event triggered when the player dies.
#[derive(Event)]
pub struct PlayerDiedEvent {
    pub player: Entity,
}

/// Generic game objective enum. More types can be added in the future.
#[derive(Deserialize, Clone, Debug)]
pub enum Objective {
    Survive(SurvivalObjective),
}

impl Objective {
    /// Returns a human-readable name for the objective.
    pub fn name(&self) -> &str {
        match self {
            Objective::Survive(_) => "Survive",
        }
    }
}

/// Survive objective: survive for a given time while eliminating all enemies.
#[derive(Deserialize, Clone, Debug)]
pub struct SurvivalObjective {
    /// Target time (in seconds) to survive.
    pub max_time: f32,
    /// Initial number of enemies.
    pub total_enemies: u32,

    /// (Runtime) Time elapsed since the objective started.
    #[serde(skip_serializing)]
    pub elapsed: f32,
    /// (Runtime) Number of enemies remaining.
    #[serde(skip_serializing)]
    pub remaining_enemies: u32,
    /// (Runtime) Whether the player is still alive.
    #[serde(skip_serializing)]
    pub player_alive: bool,
}

impl SurvivalObjective {
    /// Creates a new survival objective, initializing both config and runtime fields.
    pub fn new(max_time: f32, total_enemies: u32) -> Self {
        Self {
            max_time,
            total_enemies,
            elapsed: 0.0,
            remaining_enemies: total_enemies,
            player_alive: true,
        }
    }

    /// Advances the timer by the given delta time (in seconds).
    pub fn tick(&mut self, delta: f32) {
        self.elapsed += delta;
    }

    /// Called when an enemy is defeated, updating the remaining enemy count.
    pub fn on_enemy_defeated(&mut self) {
        self.remaining_enemies = self.remaining_enemies.saturating_sub(1);
    }

    /// Called when the player dies, updating the alive flag.
    pub fn on_player_died(&mut self) {
        self.player_alive = false;
    }

    /// Returns `true` if the objective has failed.
    ///
    /// Fails if the player dies or the timer runs out while enemies remain.
    pub fn is_failed(&self) -> bool {
        !self.player_alive
            || (self.elapsed >= self.max_time && self.remaining_enemies > 0)
    }

    /// Returns `true` if the objective is successfully completed.
    ///
    /// Completes when the timer runs out and no enemies remain.
    pub fn is_completed(&self) -> bool {
        self.elapsed >= self.max_time && self.remaining_enemies == 0
    }

    /// Returns progress as a float in the range [0.0, 1.0].
    ///
    /// Combines time and enemy defeat progress equally.
    pub fn progress(&self) -> f32 {
        let time_ratio = (self.elapsed / self.max_time).min(1.0);
        let enemy_ratio = if self.total_enemies > 0 {
            1.0 - (self.remaining_enemies as f32 / self.total_enemies as f32)
        } else {
            1.0
        };
        (time_ratio + enemy_ratio) * 0.5
    }
}
