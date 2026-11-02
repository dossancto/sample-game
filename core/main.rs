use bevy::{
    color::palettes::tailwind,
    gltf::{GltfPlugin, convert_coordinates::GltfConvertCoordinates},
    prelude::*,
};
use bevy_trenchbroom::prelude::*;
use player_gateway::Player;

#[point_class(base(Transform, Visibility))]
#[reflect(Component)]
struct Chair;

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
