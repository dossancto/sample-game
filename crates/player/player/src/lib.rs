use bevy::prelude::*;

pub mod camera;
pub mod moviment;
pub mod cursor;

pub fn plugin(app: &mut App) {
    app.add_plugins((camera::plugin, moviment::plugin, cursor::plugin));
}

