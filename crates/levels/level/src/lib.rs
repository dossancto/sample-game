use bevy::prelude::*;

pub(crate) mod editors;
pub mod props;

pub fn plugin(app: &mut App) {
    app.add_plugins((editors::plugin, props::plugin));
}
