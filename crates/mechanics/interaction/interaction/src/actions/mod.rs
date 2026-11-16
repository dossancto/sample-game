use avian3d::prelude::SpatialQuery;
use avian3d::prelude::SpatialQueryFilter;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::Cancel;
use bevy_enhanced_input::prelude::Ongoing;
use interaction_gateway::configurations::player::InteractionSourceCamera;
use interaction_gateway::configurations::player::InterationConfiguration;
use interaction_gateway::extensions::add_interactable::ReadyForInteract;
use interaction_gateway::handlers::collect::TriggerInteraction;
use interaction_gateway::{
    actions::InteractionAction,
    extensions::add_interactable::{Interactable, Interacting},
};
use player_gateway::Player;
use third_parties::packages::avian3d::CollisionLayer;
use utils::find_in_ancestors;

#[derive(Debug, Component, Default)]
pub(crate) struct InteractionLocked;

/// Registers interaction-related observers and systems with the application.
///
/// This function integrates the interaction plugin into the provided `App` instance by:
/// - Adding observers for active and cancelled interaction events.
/// - Registering the system responsible for loading items that can be interacted with during the fixed update phase.
///
/// # Parameters
/// - `app`: The application instance to which the interaction plugin will be attached. Must be a mutable reference to an `App`.
///
/// # Panics
/// Panics if the provided `app` is in an invalid state for observer or system registration.
pub(super) fn plugin(app: &mut App) {
    app.add_observer(active_interaction);
    app.add_observer(cancel_interaction);

    app.add_systems(Update, load_interacting_items);
}

/// Handles cancellation of ongoing interactions.
///
/// This observer responds to `Cancel<InteractAction>` events by removing the `Interacting` component from entities that are currently interacting, provided their interaction timer has not yet finished. This ensures that cancelled interactions are properly cleaned up and do not persist erroneously.
///
/// # Parameters
/// - `_on`: Event trigger for cancelled interaction actions. The value is not used directly.
/// - `interactables_q`: Query yielding tuples of entity and mutable reference to their `Interacting` component. Only entities currently interacting are included.
/// - `commands`: Command buffer for issuing entity modifications.
///
/// # Edge Cases
/// - Entities whose `time_to_interact` timer is already finished are ignored and not modified.
///
/// # Panics
/// Panics if the query or command buffer is invalid or if entity removal fails unexpectedly.
fn cancel_interaction(
    _on: On<Cancel<InteractionAction>>,
    interactables_q: Query<(
        Entity,
        Option<&Interacting>,
        Option<&InteractionLocked>,
        &Interactable,
    )>,
    mut commands: Commands,
) {
    for (entity, interacting, locked, interactable) in interactables_q.iter() {

        if locked.is_some() {
            commands.entity(entity).remove::<InteractionLocked>();
        }

        let Some(interacting) = interacting else {
            continue;
        };

        if interacting.time_to_interact.is_finished() {
            continue;
        }
        if interactable.should_reset_interaction_timer_on_cancel {
            commands.entity(entity).remove::<Interacting>();
        } else {
            commands.entity(entity).insert(InteractionLocked);
        }
    }
}

/// Processes ongoing interaction attempts and manages interaction state transitions.
///
/// This observer responds to `Ongoing<InteractAction>` events, performing raycasts from the player's camera to detect interactable entities within range. If an interactable is detected:
/// - If the entity is already interacting and its timer has finished, the interaction is completed and a collection trigger is issued.
/// - If the entity is not yet interacting, the `Interacting` component is added to begin the interaction process.
///
/// # Parameters
/// - `_on`: Event trigger for ongoing interaction actions. The value is not used directly.
/// - `interactables_q`: Query yielding tuples of (`CanInteract`, optional `Interacting`, and `Interactable`) for entities that can be interacted with.
/// - `interation_configuration`: Singleton reference to the interaction configuration, specifying parameters such as interaction distance.
/// - `player`: Singleton containing the player's global transform and entity, filtered to those with an `InteractionSourceCamera`.
/// - `spatial_query`: Provides spatial queries such as raycasting for detecting interactable entities.
/// - `player_collider`: Singleton entity representing the player's collider, filtered to those with the `Player` component.
/// - `commands`: Command buffer for issuing entity modifications and triggers.
///
/// # Returns
/// This function does not return a value. It issues commands to modify entity components and trigger events as needed.
///
/// # Edge Cases
/// - If no interactable entity is hit by the raycast, the function exits without making changes.
/// - If the hit entity does not match the query for interactables, the function exits without making changes.
/// - If the entity is already interacting but its timer is not finished, no action is taken.
///
/// # Panics
/// Panics if required singletons or queries are missing, or if command buffer operations fail unexpectedly.
fn active_interaction(
    _on: On<Ongoing<InteractionAction>>,
    interactables_q: Query<(
        &ReadyForInteract,
        Option<&Interacting>,
        Option<&InteractionLocked>,
        &Interactable,
    )>,
    parents_q: Query<&ChildOf>,
    interation_configuration: Single<&InterationConfiguration>,
    player: Single<(&GlobalTransform, Entity), With<InteractionSourceCamera>>,
    spatial_query: SpatialQuery,
    player_collider: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    let (player, _) = player.into_inner();

    let camera_transform = player.compute_transform();

    let interation_configuration = interation_configuration.into_inner();

    let forward = camera_transform.forward();

    let hit = spatial_query.cast_ray(
        camera_transform.translation,
        forward,
        interation_configuration.interaction_distance,
        true,
        &SpatialQueryFilter::from_mask(CollisionLayer::Interactable)
            .with_excluded_entities([*player_collider]),
    );

    let Some(hit) = hit else {
        return;
    };

    let Some((entity, (_is_hitteable, interacting, locked, interactable))) =
        find_in_ancestors(hit.entity, &interactables_q, &parents_q)
    else {
        return;
    };

    // The button is still held after the interaction completed. Do not start
    // another interaction until the corresponding Cancel event releases it.
    if locked.is_some() {
        return;
    }

    if let Some(interacting) = interacting {
        if interacting.time_to_interact.is_finished() == false {
            return;
        }

        commands.entity(entity).remove::<Interacting>();
        commands.entity(entity).insert(InteractionLocked);
        commands.trigger(TriggerInteraction { entity });
    } else {
        commands
            .entity(entity)
            .insert_if_new(Interacting::new(interactable.time_to_interact));
    }
}

/// Advances the interaction timers for all entities currently in the process of interacting.
///
/// This function is intended to be called each frame to update the progress of ongoing interactions.
/// It iterates through all entities with an `Interacting` component, and for each one whose interaction timer
/// has not yet completed, it advances the timer by the elapsed frame time.
///
/// # Parameters
/// - `interactables_q`: A mutable query over all entities and their associated `Interacting` components.
///   Each tuple contains the entity and a mutable reference to its `Interacting` state. The query must be valid and not borrowed elsewhere.
/// - `time`: A resource providing timing information, specifically the delta time since the last frame. Must not be null.
///
/// # Behavior
/// - For each entity, if its `time_to_interact` timer is already finished, the function skips further processing for that entity.
/// - Otherwise, the timer is advanced by the current frame's delta time.
///
/// # Panics & Edge Cases
/// - Panics if the query or time resource is unavailable or borrowed mutably elsewhere (as per Bevy's ECS borrowing rules).
/// - If `time.delta()` returns zero, timers will not advance for that frame.
///
/// # Usage
/// Call this function as part of your game's update loop to ensure that interaction timers progress correctly for all relevant entities.
fn load_interacting_items(
    mut interactables_q: Query<
        (Entity, &mut Interacting),
        (Without<InteractionLocked>, With<ReadyForInteract>),
    >,
    time: Res<Time>,
) {
    for (_entity, mut interacting) in interactables_q.iter_mut() {
        if interacting.time_to_interact.is_finished() {
            continue;
        }
        interacting.time_to_interact.tick(time.delta());
    }
}
