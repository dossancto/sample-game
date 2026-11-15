use std::time::Duration;

use bevy::prelude::*;

use crate::handlers::collect::interact_with_item;

/// Provides the ability to register a type as a collectable item within an application.
///
pub trait AddInteractable {
    /// Registers a type `T` as a collectable item for the implementing object.
    ///
    /// # Type Parameters
    /// * `T`: The type to be registered as collectable. Must implement [`CanBeCollect`].
    ///
    /// # Returns
    /// A mutable reference to the implementor, allowing for method chaining.
    ///
    /// # Usage
    /// Use this method to extend the set of types that the application can observe and collect.
    ///
    /// # Errors
    /// This method does not return errors directly, but the underlying implementation may panic if the observer registration fails or if `self` is in an invalid state.
    fn add_interaction<T>(&mut self) -> &mut Self
    where
        T: CanInteract;
}

impl AddInteractable for App {
    /// Registers a type `T` as a collectable item for the application instance.
    ///
    /// # Type Parameters
    /// * `T`: The type to be registered as collectable. Must implement [`CanBeCollect`].
    ///
    /// # Returns
    /// A mutable reference to the `App` instance, enabling method chaining.
    ///
    /// # Panics
    /// Panics if observer registration fails or if the application is in an invalid state for modification.
    fn add_interaction<T>(&mut self) -> &mut Self
    where
        T: CanInteract,
    {
        self.add_observer(interact_with_item::<T>);
        self
    }
}

#[derive(Component)]
pub struct Interactable {
    /// Indicates whether interaction with this entity is currently prevented.
    ///
    /// When `true`, attempts to interact with the entity will be ignored or denied.
    /// When `false`, the entity is available for interaction.
    pub blocked: bool,

    /// The minimum amount of time, in Seconds, required to complete an interaction with this entity.
    ///
    /// Use this value to implement timed interactions, such as holding a button or waiting for an action to finish.
    /// Must be a non-negative floating-point value. Negative values may result in undefined behavior.
    pub time_to_interact: Duration,

    /// Determines whether the interaction timer should reset when an interaction is cancelled.
    ///
    /// If `true`, cancelling an interaction will reset the timer, requiring the full interaction time to be completed again for the next attempt.
    /// If `false`, the timer will not reset, allowing the player to resume the interaction without
    /// starting over.
    pub should_reset_interaction_timer_on_cancel: bool,
}

impl Default for Interactable {
    fn default() -> Self {
        Self {
            blocked: false,
            time_to_interact: Duration::from_secs(0),
            should_reset_interaction_timer_on_cancel: true,
        }
    }
}

pub trait CanInteract: Component + Clone {}

#[derive(Component)]
pub struct ReadyForInteract;

#[derive(Component)]
pub struct Interacting {
    pub time_to_interact: Timer,
}

impl Interacting {
    pub fn new(duration: Duration) -> Self {
        Self {
            time_to_interact: Timer::from_seconds(duration.as_secs_f32(), TimerMode::Once),
        }
    }
}
