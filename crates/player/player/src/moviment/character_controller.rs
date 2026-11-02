use bevy::prelude::*;
use bevy_ahoy::CharacterController;
use player_gateway::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_character_controller);
}

fn spawn_character_controller(_on: On<Add, Player>, mut commands: Commands) {
    commands.spawn((
        CharacterController::default(),
        Transform::from_xyz(0.0, 20.0, 0.0),
    ));
}
