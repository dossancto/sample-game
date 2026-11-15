use bevy::prelude::*;
use bevy_ahoy::camera::CharacterControllerCameraOf;
use interaction_gateway::configurations::player::InteractionSourceCamera;
use player_gateway::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_player_camera);
}

fn spawn_player_camera(on: On<Add, Player>, mut commands: Commands) {
    let player = on.entity;

    commands.spawn((
        InteractionSourceCamera,
        Camera3d::default(),
        CharacterControllerCameraOf::new(player),
    ));
}
