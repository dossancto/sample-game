use bevy::prelude::*;

pub mod general;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((general::plugin,));
}
