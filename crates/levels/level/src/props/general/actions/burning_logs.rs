use avian3d::prelude::{ColliderConstructor, RigidBody};
use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;
use interaction_gateway::extensions::add_interactable::{
    AddInteractable, CanInteract, Interactable,
};
use level_utils::editors::trenchbroom::spawning::setup_prop;
use money_gateway::CollectableMoney;

pub(super) fn plugin(app: &mut App) {
    app.add_interaction::<BurningLogs>();
    app.add_observer(add_behavior);
}

#[point_class(
    base(Transform, Visibility),
    model(
        "models/darkmod/fireplace/burntwood.gltf"
    )
)]
#[derive(Clone)]
#[reflect(Component)]
#[component(on_add = setup_prop::<BurningLogs>(RigidBody::Dynamic, ColliderConstructor::ConvexHullFromMesh))]
pub struct BurningLogs;

impl CanInteract for BurningLogs {}

fn add_behavior(on: On<Add, BurningLogs>, mut commands: Commands) {
    commands.entity(on.entity).insert_if_new((
        Interactable { ..default() },
        CollectableMoney {
            value: 1f64,
        },
    ));
}
