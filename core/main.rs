use avian3d::prelude::*;
use bevy::{
    asset::io::embedded::GetAssetServer,
    color::palettes::tailwind,
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    gltf::{GltfPlugin, convert_coordinates::GltfConvertCoordinates},
    prelude::*,
    scene::ResolvedScene,
};
use bevy_trenchbroom::prelude::*;
use level_gateway::CollisionLayer;
use player_gateway::Player;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/furniture/seating/wchair1.gltf")
)]
#[reflect(Component)]
#[component(on_add = setup_prop::<Chair>(RigidBody::Dynamic, ColliderConstructor::ConvexHullFromMesh))]
struct Chair;

#[point_class(
    base(Transform, Visibility),
    model(
        "models/darkmod/lights/non-extinguishable/round_lantern_sitting/round_lantern_sitting.gltf"
    )
)]
#[reflect(Component)]
struct LampSitting;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(GltfPlugin {
                convert_coordinates: GltfConvertCoordinates {
                    rotate_scene_entity: true,
                    rotate_meshes: true,
                },
                ..default()
            }),
            third_parties::plugin,
            level::plugin,
            player::plugin,
            keybinds::plugin,
        ))
        .insert_resource(ClearColor(tailwind::SKY_200.into()))
        .add_systems(Startup, setup)
        .add_observer(add_player)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(WorldAssetRoot(assets.load("maps/test/test.map#Scene")));
}

fn add_player(insert: On<Insert, Chair>, mut commands: Commands, spawner: Query<&Transform>) {
    let Ok(transform) = spawner.get(insert.entity).copied() else {
        return;
    };

    commands.spawn((Player, transform));
}

fn setup_prop<T: QuakeClass>(
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
            world
                .entity_mut(ctx.entity)
                .insert((ColliderConstructorHierarchy::new(
                    ColliderConstructor::ConvexHullFromMesh,
                )
                .with_default_layers(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))
                .with_default_density(1_000.0),));
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

fn quake_bundle<T: QuakeClass>(
    asset_server: AssetServer,
    rigid_body: RigidBody,
    constructor: ColliderConstructor,
) -> impl Bundle {
    let model = asset_server.load_trenchbroom_model::<T>();

    (
        ColliderConstructorHierarchy::new(constructor)
            .with_default_layers(CollisionLayers::new(CollisionLayer::Prop, LayerMask::ALL))
            // About the density of oak wood (600-800 kg/m^3)
            .with_default_density(800.0),
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
