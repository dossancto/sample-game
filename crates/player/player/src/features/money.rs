use bevy::prelude::*;
use money_gateway::MoneyBag;
use player_gateway::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(add_money_to_player);
}

fn add_money_to_player(on: On<Add, Player>, mut commands: Commands) {
    commands
        .entity(on.entity)
        .insert_if_new((MoneyBag::default(),));
}
