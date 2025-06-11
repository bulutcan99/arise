use bevy::prelude::*;
use engine::combat::{DamageDealtEvent, HealthRegainResetEvent};
use engine::health::{HealthComponent, HealthRegainComponent};
use engine::player::PlayerComponent;

pub mod light_attack;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageDealtEvent>()
            .add_event::<HealthRegainResetEvent>()
            .add_systems(
                Update,
                (
                    damage_system,
                    regenerate_health_system,
                    reset_regenerate_health_system,
                ),
            );
    }
}

/// System that handles health regeneration over time for player entities only.
///
/// This systems simulates a delayed regeneration mechanism, where a player begins
/// to regain health gradually if they haven't taken damage for a specified period.
///
/// Internally, it uses two timers per player:
///
/// - `delay_timer`: Starts ticking after the player takes damage. Health regeneration
///   will not begin until this timer finishes.
/// - `interval_timer`: Once the delay is over, this timer controls how often health is restored.
///
/// When the delay timer completes, the interval timer starts ticking. Every time it finishes,
/// a fixed amount of health is restored, as specified in the `HealthRegainComponent`.
///
/// This systems assumes that the `HealthRegainComponent::reset()` method is called elsewhere (e.g., in the
/// `damage_system`) to restart the delay timer whenever the player takes damage.
///
/// ### Components required per entity:
/// - [`HealthComponent`] — Tracks the entity's current and maximum health.
/// - [`HealthRegainComponent`] — Manages regeneration timers and heal amount.
/// - [`PlayerComponent`] — Marker component to filter only player entities.
///
/// ### Parameters:
/// - `time`: Global [`Time`] resource used to advance timers.
/// - `query`: Only player entities (`With<PlayerComponent>`) that have both `HealthComponent`
///   and `HealthRegainComponent`.
///
/// ### Example Flow:
/// ```text
/// Player takes damage → delay timer is reset.
/// After delay duration passes without further damage → interval timer starts ticking.
/// Each time interval finishes → player heals by a fixed amount (e.g., +1 HP).
/// ```
fn regenerate_health_system(
    time: Res<Time>,
    mut query: Query<
        (
            &mut HealthComponent,
            &mut HealthRegainComponent,
        ),
        With<PlayerComponent>,
    >,
) {
    for (mut health, mut regain) in query.iter_mut() {
        regain.update(time.delta(), &mut health);
    }
}

/// System that resets the health regeneration state for player entities upon receiving a reset event.
///
/// This systems listens for [`HealthRegainResetEvent`] events, which are typically emitted
/// when an entity takes damage. Upon receiving such an event, it checks whether the targeted
/// entity is a player (via [`PlayerComponent`]) and, if so, resets its [`HealthRegainComponent`].
///
/// Resetting the component restarts the internal delay and interval timers, ensuring that
/// health regeneration is paused and will only begin again after a new delay period.
///
/// ### Components required per entity:
/// - [`HealthRegainComponent`] — Manages regeneration logic and timers.
/// - [`PlayerComponent`] — Used to filter the affected entities to only players.
///
/// ### Parameters:
/// - `time`: Global [`Time`] resource (unused here but kept for consistency or future use).
/// - `health_regain_reset_events`: Reads the queue of incoming `HealthRegainResetEvent`s.
/// - `query`: Targets player entities that have `HealthRegainComponent`.
///
/// ### Example Flow:
/// ```text
/// Player takes damage → `HealthRegainResetEvent` is emitted → this systems resets regeneration.
/// ```
///
/// This systems ensures that only player entities respond to the regeneration reset logic.
fn reset_regenerate_health_system(
    time: Res<Time>,
    mut health_regain_reset_events: EventReader<HealthRegainResetEvent>,
    mut query: Query<&mut HealthRegainComponent, With<PlayerComponent>>,
) {
    for event in health_regain_reset_events.read() {
        if let Ok(mut regain) = query.get_mut(event.entity) {
            regain.reset();
        }
    }
}

/// System that processes damage events and applies damage to target entities' health.
///
/// This systems listens to [`DamageDealtEvent`] events and, for each event:
/// - Fetches the corresponding target entity using its [`Entity`] ID.
/// - Applies damage to the entity's [`HealthComponent`] by calling `.take_damage()`.
/// - Optionally, this is a good place to trigger visual/audio feedback effects (e.g., hit animations, particles, sound).
///
/// ### Components required per target entity:
/// - [`HealthComponent`] — stores the entity's health and applies the damage logic.
///
/// ### Parameters:
/// - `time`: The global [`Time`] resource (not currently used but available for effect timing if needed).
/// - `damage_dealt_event`: A queue of [`DamageDealtEvent`] instances emitted elsewhere in the game (e.g., on collisions or attacks).
/// - `query`: Used to look up entities by their [`Entity`] ID and mutate their health.
///
/// ### Example flow:
/// ```text
/// Player attacks enemy → emits DamageDealtEvent(target = enemy_entity, damage = 10)
/// System reads event → applies 10 damage to enemy_entity's HealthComponent
/// ```
///
/// ### TODO:
/// - Trigger hit effects (e.g., particles, screen shake, sound) based on `Transform` position.
/// -
fn damage_system(
    time: Res<Time>,
    mut damage_dealt_events: EventReader<DamageDealtEvent>,
    mut health_regain_reset_events: EventWriter<HealthRegainResetEvent>,
    mut query: Query<(Entity, &mut HealthComponent)>,
) {
    for event in damage_dealt_events.read() {
        if let Ok((_entity, mut health_component)) = query.get_mut(event.target)
        {
            health_component.take_damage(event.damage);

            health_regain_reset_events.send(HealthRegainResetEvent {
                entity: event.target,
            });

            // TODO: Add visual/audio feedback effect at the entity's position
        }
    }
}
