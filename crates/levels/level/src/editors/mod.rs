use bevy::prelude::*;

pub(crate) mod trenchbroom;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((trenchbroom::plugin,));
}
