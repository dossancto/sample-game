use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_ahoy::CharacterController;
use level_gateway::CollisionLayer;
use player_gateway::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_character_controller);
}

fn spawn_character_controller(on: On<Add, Player>, mut commands: Commands) {
    let player = on.entity;

    commands.entity(player).insert((
        CharacterController::default(),
        RigidBody::Kinematic,
        Collider::cylinder(0.7, 1.8),
        CollisionLayers::new([CollisionLayer::Player], LayerMask::ALL),
        Transform::from_xyz(0.0, 20.0, 0.0),
    ));
}
