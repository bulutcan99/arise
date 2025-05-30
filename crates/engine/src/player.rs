use bevy::prelude::*;

use crate::character::{Character, CharacterType};
use crate::spwanable::SpawnPosition;

/// Stores all active and potential player slots.
#[derive(Resource, Debug, Default)]
pub struct PlayersResource {
    /// List of player slots. A slot is `Some(PlayerData)` if a player has joined, `None` otherwise.
    pub player_data: Option<PlayerData>,
}

/// Represents a player’s configuration for a slot, including chosen character and input method.
#[derive(Debug, Clone)]
pub struct PlayerData {
    /// The character selected by the player.
    pub character: CharacterType,
}

/// Bundle containing all necessary components to represent a player entity.
#[derive(Bundle)]
pub struct PlayerBundle {
    id: PlayerIDComponent,
    flag: PlayerComponent,
    movement: PlayerMobilityComponent,
    outgoing_damage: PlayerOutgoingDamageComponent,
    incoming_damage: PlayerIncomingDamageComponent,
}

impl PlayerBundle {
    pub fn with_id(self, id: PlayerIDComponent) -> Self {
        Self { id, ..self }
    }
}

impl From<&Character> for PlayerBundle {
    fn from(character: &Character) -> Self {
        Self {
            id: PlayerIDComponent::One,
            movement: character.into(),
            outgoing_damage: character.into(),
            incoming_damage: PlayerIncomingDamageComponent::default(),
            flag: PlayerComponent,
        }
    }
}

/// Uniquely identifies a player for logic/UI syncing purposes.
#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum PlayerIDComponent {
    One,
    Two,
}

/// Marker component to indicate that an entity is a player.
#[derive(Component)]
pub struct PlayerComponent;

/// Component defining player movement attributes and capabilities.
#[derive(Component)]
pub struct PlayerMobilityComponent {
    /// Acceleration vector applied while moving.
    pub acceleration: Vec2,
    /// Deceleration vector applied when not receiving movement input.
    pub deceleration: Vec2,
    /// Maximum movement speed vector.
    pub speed: Vec2,
    /// Size of the collider used for physics and collision detection.
    pub collider_dimensions: Vec2,
    /// Whether movement is currently enabled for this player.
    pub movement_enabled: bool,
}

impl From<&Character> for PlayerMobilityComponent {
    fn from(character: &Character) -> Self {
        Self {
            acceleration: character.acceleration,
            deceleration: character.deceleration,
            speed: character.speed,
            collider_dimensions: character.collider_dimensions,
            movement_enabled: true,
        }
    }
}

/// Component representing the player's offensive stats and projectile behavior.
#[derive(Component)]
pub struct PlayerOutgoingDamageComponent {
    /// Damage dealt when colliding directly with an enemy.
    pub collision_damage: u32,
    /// Base damage dealt through weapon abilities.
    pub weapon_damage: u32,
    /// Speed at which projectiles travel.
    pub projectile_speed: f32,
    /// Initial spawn position offset for weapon projectiles.
    pub projectile_spawn_position: SpawnPosition,
    /// Time (in seconds) after which projectiles will be despawned.
    pub projectile_despawn_time: f32,
    /// Visual and physical size of projectiles.
    pub projectile_size: f32,
    /// Number of projectiles fired per ability activation.
    pub projectile_count: u32,
}

impl From<&Character> for PlayerOutgoingDamageComponent {
    fn from(character: &Character) -> Self {
        Self {
            collision_damage: character.collision_damage,
            weapon_damage: character.weapon_damage,
            projectile_speed: character.projectile_speed,
            projectile_spawn_position: character
                .projectile_spawn_position
                .clone(),
            projectile_despawn_time: character.projectile_despawn_time,
            projectile_size: character.projectile_size,
            projectile_count: character.projectile_count,
        }
    }
}

/// Component defining how much incoming damage is modified for the player.
#[derive(Component)]
pub struct PlayerIncomingDamageComponent {
    /// Multiplier applied to all incoming damage (e.g. 1.0 = normal, 0.5 = half damage).
    pub multiplier: f32,
}

impl Default for PlayerIncomingDamageComponent {
    fn default() -> Self {
        Self { multiplier: 1.0 }
    }
}
