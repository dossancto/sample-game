use avian3d::prelude::*;
use bevy::{
    asset::io::embedded::GetAssetServer,
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_trenchbroom::prelude::*;
use third_parties::packages::avian3d::CollisionLayer;

pub fn setup_prop<T: QuakeClass>(
    rigid_body: RigidBody,
    collider: ColliderConstructor,
) -> impl FnOnce(DeferredWorld, HookContext) {
    move |mut world, ctx| {
        if world.is_scene_world() {
            return;
        }
        world.commands().queue(move |world: &mut World| {
            let asset_server = world.get_asset_server().clone();

            world.entity_mut(ctx.entity).insert(quake_bundle::<T>(
                asset_server,
                rigid_body,
                collider,
            ));
        });
    }
}

pub trait GetTrenchbroomModelPath: QuakeClass {
    fn model_path() -> String {
        Self::CLASS_INFO.model_path().unwrap().to_string()
    }
    fn scene_path() -> String {
        format!("{file_path}#Scene0", file_path = Self::model_path())
    }
    fn animation_path(index: u32) -> String {
        format!(
            "{file_path}#Animation{index}",
            file_path = Self::model_path()
        )
    }
}

pub fn quake_bundle<T: QuakeClass>(
    asset_server: AssetServer,
    rigid_body: RigidBody,
    constructor: ColliderConstructor,
) -> impl Bundle {
    let model = asset_server.load_trenchbroom_model::<T>();

    (
        ColliderConstructorHierarchy::new(constructor)
            .with_default_layers(CollisionLayers::new(
                CollisionLayer::Interactable,
                LayerMask::ALL,
            ))
            .with_default_density(1_000.0),
        rigid_body,
        WorldAssetRoot(model),
    )
}

impl<T: QuakeClass> GetTrenchbroomModelPath for T {}

pub trait LoadTrenchbroomModel {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<WorldAsset>;
}

impl LoadTrenchbroomModel for DeferredWorld<'_> {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<WorldAsset> {
        self.resource::<AssetServer>().load_trenchbroom_model::<T>()
    }
}

impl LoadTrenchbroomModel for AssetServer {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<WorldAsset> {
        self.load(T::scene_path())
    }
}
