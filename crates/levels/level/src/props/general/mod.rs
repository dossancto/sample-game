use bevy::prelude::*;

pub mod actions;
pub mod model;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((actions::plugin,));
}
