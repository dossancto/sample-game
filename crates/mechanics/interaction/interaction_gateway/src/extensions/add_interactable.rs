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
    pub blocked: bool,
}

impl Default for Interactable {
    fn default() -> Self {
        Self { blocked: false }
    }
}

pub trait CanInteract: Component + Clone {}

#[derive(Component)]
pub struct ReadyForInteract;

#[derive(Component)]
pub struct Interacting {
    pub time_to_interact: Timer,
}
