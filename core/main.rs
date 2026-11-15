use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind,
    gltf::{GltfPlugin, convert_coordinates::GltfConvertCoordinates},
    prelude::*,
};
use bevy_trenchbroom::prelude::*;
use level_gateway::Screen;
use level_utils::editors::trenchbroom::spawning::setup_prop;
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
#[component(on_add = setup_prop::<LampSitting>(RigidBody::Dynamic, ColliderConstructor::ConvexHullFromMesh))]
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
            interaction::plugin,
        ))
        .insert_resource(ClearColor(tailwind::SKY_800.into()))
        .add_systems(Startup, setup)
        .add_observer(add_player)
        .init_state::<Screen>()
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(WorldAssetRoot(assets.load("maps/test/test.map#Scene")));
}

fn add_player(
    // TODO: Change the Chair to some player spawner
    insert: On<Insert, Chair>,
    mut next_screen: ResMut<NextState<Screen>>,
    mut commands: Commands,
    spawner: Query<&Transform>,
) {
    let Ok(transform) = spawner.get(insert.entity).copied() else {
        return;
    };

    next_screen.set(Screen::Gameplay);
    commands.spawn((Player, transform));
}
