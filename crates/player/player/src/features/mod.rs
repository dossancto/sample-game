use bevy::prelude::*;

pub mod bag;
pub mod money;
pub mod aim_grid;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((bag::plugin, money::plugin, aim_grid::plugin));
}
