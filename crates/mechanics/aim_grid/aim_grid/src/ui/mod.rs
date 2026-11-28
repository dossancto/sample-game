use bevy::prelude::*;

pub mod grid;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((grid::plugin,));
}
