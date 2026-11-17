use bevy::prelude::*;

pub mod handlers;

pub fn plugin(app: &mut App) {
    app.add_plugins((handlers::plugin,));
}
