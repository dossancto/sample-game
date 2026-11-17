use bevy::prelude::*;

pub mod bag;
pub mod money;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((bag::plugin, money::plugin));
}
