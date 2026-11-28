use bevy::prelude::*;
use bevy_enhanced_input::action::events::{Cancel, Ongoing};
use player_gateway::{Player, actions::PlayerMoveFoward, components::PlayerMoving};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_walk);
    app.add_observer(on_stop_walk);

    app.add_systems(Update, player_moving);
}

fn on_walk(
    _on: On<Ongoing<PlayerMoveFoward>>,
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
    player: Single<(&mut Transform,), (With<Player>, With<PlayerMoving>)>,
    time: Res<Time>,
) {
    let (mut player_transform,) = player.into_inner();

    let foward = player_transform.forward();

    // TODO: Put into a resource or configuration
    let speed = 10f32;

    player_transform.translation += *foward * speed * time.delta_secs();
}
