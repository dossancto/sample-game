use bevy::prelude::*;

mod editors;

pub fn plugin(app: &mut App) {
    app.add_plugins((editors::plugin,));
}
