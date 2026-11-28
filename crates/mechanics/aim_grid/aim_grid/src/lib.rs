use bevy::prelude::*;

pub mod debug;
pub mod moviment;
pub mod ui;

pub fn plugin(app: &mut App) {
    app.add_plugins((moviment::plugin, debug::plugin, ui::plugin));
}
