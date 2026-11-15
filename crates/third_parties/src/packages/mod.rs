use bevy::prelude::*;

pub mod avian3d;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((avian3d::plugin,));
}
