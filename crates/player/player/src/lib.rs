use bevy::prelude::*;

pub mod camera;
pub mod cursor;
pub mod features;
pub mod moviment;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        camera::plugin,
        moviment::plugin,
        cursor::plugin,
        features::plugin,
    ));
}
