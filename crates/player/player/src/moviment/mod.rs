use bevy::prelude::*;

mod character_controller;
mod walk;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((character_controller::plugin, walk::plugin));
}
