use avian3d::prelude::{ColliderConstructor, RigidBody};
use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;
use interaction_gateway::extensions::add_interactable::CanInteract;
use level_utils::editors::trenchbroom::spawning::setup_prop;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/furniture/seating/wchair1.gltf")
)]
#[derive(Clone)]
#[reflect(Component)]
#[component(on_add = setup_prop::<Chair>(RigidBody::Dynamic, ColliderConstructor::ConvexHullFromMesh))]
pub struct Chair;

impl CanInteract for Chair {}
