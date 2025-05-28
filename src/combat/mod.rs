use bevy::prelude::*;
use engine::combat::{DamageDealtEvent, HealthRegainResetEvent};
use engine::health::{HealthComponent, HealthRegainComponent};
use engine::player::PlayerComponent;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageDealtEvent>()
            .add_event::<HealthRegainResetEvent>()
            .add_systems(Update, (damage_system, regenerate_health_system));
    }
}

/// System that handles health regeneration for entities over time.
///
/// This system ticks two internal timers per entity:
///
/// - `delay_timer`: Defines how long the entity must go without taking damage before regeneration starts.
/// - `interval_timer`: Once the delay is passed, defines how frequently the entity regains health.
///
/// When the delay timer finishes, the interval timer begins ticking. Every time the interval timer finishes,
/// the entity regains a fixed amount of health, defined by its `HealthRegainComponent`.
///
/// This system assumes that the entity's `HealthRegainComponent` is reset (via `.reset()`) whenever it takes damage,
/// effectively restarting the delay period.
///
/// ### Components required per entity:
/// - [`HealthComponent`] — stores the current and maximum health values.
/// - [`HealthRegainComponent`] — manages the regeneration logic (timers and regeneration amount).
///
/// ### Parameters:
/// - `time`: Global [`Time`] resource used to tick timers.
/// - `query`: Iterates over all entities with both `HealthComponent` and `HealthRegainComponent`.
///
/// ### Example:
/// ```text
/// Entity takes damage → delay timer starts.
/// After `x` seconds without damage → interval timer begins ticking every 1 second.
/// Every time interval timer finishes → +1 health is restored.
/// ```
fn regenerate_health_system(
    time: Res<Time>,
    mut query: Query<(&mut HealthComponent, &mut HealthRegainComponent), With<PlayerComponent>>,
) {
    for (mut health, mut regain) in query.iter_mut() {
        regain.update(time.delta(), &mut health);
    }
}

/// System that processes damage events and applies damage to target entities' health.
///
/// This system listens to [`DamageDealtEvent`] events and, for each event:
/// - Fetches the corresponding target entity using its [`Entity`] ID.
/// - Applies damage to the entity's [`HealthComponent`] by calling `.take_damage()`.
/// - Optionally, this is a good place to trigger visual/audio feedback effects (e.g., hit animations, particles, sound).
///
/// ### Components required per target entity:
/// - [`HealthComponent`] — stores the entity's health and applies the damage logic.
/// - [`Transform`] — can be used later to spawn visual effects at the entity's position.
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
    mut damage_dealt_event: EventReader<DamageDealtEvent>,
    mut query: Query<(&Entity, &mut HealthComponent, &mut HealthRegainComponent, &Transform)>,
) {
    for event in damage_dealt_event.read() {
        if let Ok((_entity, mut health_component, mut health_regain_component, transform))
            = query.get_mut(event.target) {
            health_component.take_damage(event.damage);

            health_regain_component.reset();

            // TODO: Add visual/audio feedback effect at the entity's position
        }
    }
}
