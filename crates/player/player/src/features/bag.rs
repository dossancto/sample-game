use bag_gateway::components::bag::Bag;
use bevy::prelude::*;
use player_gateway::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(add_bag_to_player);
}

fn add_bag_to_player(on: On<Add, Player>, mut commands: Commands) {
    commands
        .entity(on.entity)
        .insert_if_new((Bag::new(2f64, 2f64),));
}
