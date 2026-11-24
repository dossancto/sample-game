use aim_grid_gateway::components::hand_status::HandStatusBundle;
use bevy::prelude::*;
use player_gateway::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(add_required_components);
}

fn add_required_components(on: On<Add, Player>, mut commands: Commands) {
    commands
        .entity(on.entity)
        .insert_if_new((HandStatusBundle::default(),));
}
