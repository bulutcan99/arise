use bevy::log;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::spwanable::MobType;

/// Event triggered when an entity (e.g., an enemy) is defeated.
#[derive(Event)]
pub struct EnemyDefeatedEvent {
	pub is_boss: bool,
	pub entity: Entity,
}

/// Event triggered when the player dies.
#[derive(Event)]
pub struct PlayerDiedEvent {
	pub player: Entity,
}

/// Generic game objective enum. More types can be added in the future.
#[derive(Deserialize, Serialize, Clone, Debug)]
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
/// The objective is won if all enemies are defeated *before* max_time is reached.
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct SurvivalObjective {
	/// Target time (in seconds) to survive.
	pub max_time: f32,
	/// Initial number of enemies.
	pub total_enemies: u32,

	/// (Runtime) Time elapsed since the objective started.
	/// This field is managed at runtime and should not be part of the config.
	#[serde(skip)]
	pub elapsed: f32,

	/// (Runtime) Number of enemies remaining.
	/// Initialized to `total_enemies` when the objective starts.
	#[serde(skip)]
	pub remaining_enemies: u32,

	/// (Runtime) Whether the player is still alive.
	/// Initialized to `true` when the objective starts.
	#[serde(skip)]
	pub player_alive: bool,
}

impl SurvivalObjective {
	/// Creates a new survival objective, initializing both config and runtime fields.
	/// This is typically called when the objective becomes active.
	pub fn new(max_time: f32, total_enemies: u32) -> Self {
		Self {
			max_time,
			total_enemies,
			elapsed: 0.0,
			remaining_enemies: total_enemies,
			player_alive: true,
		}
	}

	/// Call this method when the objective is deserialized and needs to be made active.
	/// It sets the runtime fields to their initial active state.
	pub fn activate(&mut self) {
		self.elapsed = 0.0;
		self.remaining_enemies = self.total_enemies;
		self.player_alive = true;
	}

	/// Advances the timer by the given delta time (in seconds).
	pub fn tick(&mut self, delta: f32) {
		if self.player_alive && !self.is_completed() {
			self.elapsed += delta;
		}
	}

	/// Called when an enemy is defeated, updating the remaining enemy count.
	pub fn on_enemy_defeated(&mut self) {
		if self.player_alive {
			self.remaining_enemies = self.remaining_enemies.saturating_sub(1);
		}
	}

	/// Called when the event `PlayerDiedEvent` trigger player dies and update the alive flag.
	pub fn on_player_died(&mut self) {
		self.player_alive = false;
	}

	/// Returns `true` if the objective is successfully completed.
	///
	/// Completes if the player is alive, all enemies are eliminated,
	/// AND the max time has NOT been reached.
	pub fn is_completed(&self) -> bool {
		self.player_alive
			&& self.remaining_enemies == 0
			&& self.elapsed < self.max_time
	}

	/// Returns `true` if the objective has failed.
	///
	/// Fails if the player dies, OR if the max_time is reached
	/// (and the objective wasn't completed before that).
	pub fn is_failed(&self) -> bool {
		if !self.player_alive {
			return true;
		}
		if self.elapsed >= self.max_time {
			return true;
		}

		false
	}

	/// Returns a combined progress as a float in the range [0.0, 1.0].
	///
	/// Represents an equally weighted combination of enemies defeated and time elapsed
	/// relative to `max_time`. Returns 1.0 if completed, and 0.0 if failed.
	pub fn progress(&self) -> f32 {
		if self.is_completed() {
			return 1.0;
		}
		if self.is_failed() {
			return 0.0;
		}

		let enemy_defeat_ratio = if self.total_enemies > 0 {
			(self.total_enemies - self.remaining_enemies) as f32
				/ self.total_enemies as f32
		} else {
			1.0
		};

		// Ratio of time elapsed towards max_time.
		// This shows how much of the "allowed" time has been used.
		let time_elapsed_ratio = (self.elapsed / self.max_time).min(1.0);

		log::info!(
            "Enemy Defeat Ratio: {}, Time Elapsed Ratio: {}",
            enemy_defeat_ratio,
            time_elapsed_ratio
        );

		// Combine the two ratios. Here, we take a simple average.
		// You might want to weigh them differently based on game feel.
		(enemy_defeat_ratio + time_elapsed_ratio) / 2.0
	}
}
