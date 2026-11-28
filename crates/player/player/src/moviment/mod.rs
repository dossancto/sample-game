use bevy::prelude::*;

mod character_controller;
mod walk;
mod camera_bounce;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((character_controller::plugin, walk::plugin, camera_bounce::plugin));
}
