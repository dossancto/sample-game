use avian3d::prelude::{ColliderConstructor, RigidBody};
use bag_gateway::components::collectable_item::CollectableItemStats;
use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;
use interaction_gateway::extensions::add_interactable::{
    AddInteractable, CanInteract, Interactable,
};
use level_utils::editors::trenchbroom::spawning::setup_prop;

pub(super) fn plugin(app: &mut App) {
    app.add_interaction::<LampSitting>();
    app.add_observer(add_behavior);
}

#[point_class(
    base(Transform, Visibility),
    model(
        "models/darkmod/lights/non-extinguishable/round_lantern_sitting/round_lantern_sitting.gltf"
    )
)]
#[derive(Clone)]
#[reflect(Component)]
#[component(on_add = setup_prop::<LampSitting>(RigidBody::Dynamic, ColliderConstructor::ConvexHullFromMesh))]
pub struct LampSitting;

impl CanInteract for LampSitting {}

fn add_behavior(on: On<Add, LampSitting>, mut commands: Commands) {
    commands.entity(on.entity).insert_if_new((
        Interactable { ..default() },
        CollectableItemStats {
            volume: 1f64,
            density: 1f64,
        },
    ));
}
