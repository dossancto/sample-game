use bevy::prelude::*;

pub mod bag;

pub fn plugin(app: &mut App) {
    app.add_plugins((bag::plugin,));
}

