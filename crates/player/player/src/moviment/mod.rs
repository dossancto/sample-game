use bevy::prelude::*;

mod character_controller;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((character_controller::plugin,));
}
