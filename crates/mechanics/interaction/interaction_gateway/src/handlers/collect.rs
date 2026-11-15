use bevy::prelude::*;

use crate::extensions::add_interactable::{CanInteract, Interactable};

#[derive(Event)]
pub struct TriggerInteraction {
    /// * `entity` - The [`Entity`] to be collected. Must be a valid entity reference; passing an invalid or despawned entity may result in no action or errors in downstream systems.
    pub entity: Entity,
}

#[derive(EntityEvent, Debug)]
pub struct InteractionEnd<E> {
    /// The entity that performed the interaction action.
    pub entity: Entity,
    /// The value or item that was collected by the entity.
    pub value: E,
}

impl<E> InteractionEnd<E> {
    pub(crate) fn new(event: E, entity: Entity) -> Self {
        InteractionEnd {
            value: event,
            entity,
        }
    }
}


pub(crate) fn interact_with_item<T: CanInteract>(
    on: On<TriggerInteraction>,
    mut commands: Commands,
    query: Query<&T, With<Interactable>>,
) {
    let entity = on.entity;

    let Ok(interactable) = query.get(entity) else {
        return;
    };

    let event_to_trigger = InteractionEnd::new(interactable.clone(), entity);

    commands.trigger(event_to_trigger);
}
