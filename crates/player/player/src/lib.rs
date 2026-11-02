use bevy::prelude::*;
use player_gateway::Player;

pub mod camera;
pub mod moviment;

pub fn plugin(app: &mut App) {
    app.add_plugins((camera::plugin, moviment::plugin));
}

