use bevy::prelude::*;
use bevy_enhanced_input::action::events::{Cancel, Start};
use player_gateway::{
    Player,
    actions::PlayerMoveFoward,
    components::{PlayerMovimentControll, PlayerMoving},
};

use crate::moviment::camera_bounce::CameraBounce;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(spawn);

    app.add_observer(on_walk);
    app.add_observer(on_stop_walk);

    app.add_systems(Update, player_moving);
}

fn spawn(on: On<Add, Player>, mut commands: Commands) {
    let player = on.entity;

    commands.entity(player).insert((
        PlayerMovimentControll { speed: 10f32 },
        CameraBounce::default(),
    ));
}

fn on_walk(
    _on: On<Start<PlayerMoveFoward>>,
    player: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    commands.entity(player.entity()).insert_if_new(PlayerMoving);
}

fn on_stop_walk(
    _on: On<Cancel<PlayerMoveFoward>>,
    player: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    commands.entity(player.entity()).remove::<PlayerMoving>();
}

fn player_moving(
    player: Single<(&mut Transform, &PlayerMovimentControll), (With<Player>, With<PlayerMoving>)>,
    time: Res<Time>,
) {
    let (mut player_transform, controll) = player.into_inner();

    let foward = player_transform.forward();

    player_transform.translation += *foward * controll.speed * time.delta_secs();
}
