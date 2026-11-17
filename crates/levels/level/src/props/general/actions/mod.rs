use bevy::prelude::*;

pub mod chair;
pub mod lamp_sitting;
pub mod burning_logs;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((chair::plugin, lamp_sitting::plugin, burning_logs::plugin));
}
