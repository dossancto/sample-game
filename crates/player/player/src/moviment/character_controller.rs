use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_ahoy::CharacterController;
use interaction_gateway::configurations::player::InterationConfiguration;
use player_gateway::Player;
use third_parties::packages::avian3d::CollisionLayer;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn_character_controller);
}

fn spawn_character_controller(on: On<Add, Player>, mut commands: Commands) {
    let player = on.entity;

    commands.entity(player).insert((
        InterationConfiguration {
            interaction_distance: 20.0,
        },
        CharacterController::default(),
        RigidBody::Kinematic,
        Collider::cylinder(0.7, 1.8),
        CollisionLayers::new([CollisionLayer::Character], LayerMask::ALL),
        Transform::from_xyz(0.0, 20.0, 0.0),
    ));
}
