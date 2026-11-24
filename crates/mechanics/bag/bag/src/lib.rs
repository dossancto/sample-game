use bevy::prelude::*;

pub mod actions;
pub mod ui;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        actions::plugin,
        // ui::plugin
    ));
}
