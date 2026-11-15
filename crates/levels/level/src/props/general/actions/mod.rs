use bevy::prelude::*;

pub mod chair;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((chair::plugin,));
}
