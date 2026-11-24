use bevy::{
    color::palettes::tailwind,
    gltf::{GltfPlugin, convert_coordinates::GltfConvertCoordinates},
    prelude::*,
};
use level::props::general::model::Chair;
use level_gateway::Screen;
use player_gateway::Player;

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
            bag::plugin,
            menus::plugin,
            money::plugin,
            aim_grid::plugin,
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
