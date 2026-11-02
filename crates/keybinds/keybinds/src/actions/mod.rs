use bevy::prelude::*;

pub(crate) mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((player::plugin,));
}
