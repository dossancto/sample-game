use bevy::prelude::*;

pub mod actions;
pub mod core;
pub mod ui;

pub fn plugin(app: &mut App) {
    app.add_plugins((core::plugin, ui::plugin, actions::plugin));
}
