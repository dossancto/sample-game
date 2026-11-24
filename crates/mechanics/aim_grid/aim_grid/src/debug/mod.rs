use bevy::prelude::*;

pub mod ui;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((ui::plugin,));
}
