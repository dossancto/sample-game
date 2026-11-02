use bevy::prelude::*;
use bevy_ahoy::camera::CharacterControllerCameraOf;
use player_gateway::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_player_camera);
}

fn spawn_player_camera(on: On<Add, Player>, mut commands: Commands) {
    let player = on.entity;

    let camera_controller = CharacterControllerCameraOf::new(player);

    commands.entity(player).insert(camera_controller);
}
