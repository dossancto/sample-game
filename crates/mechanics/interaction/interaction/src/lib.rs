use bevy::prelude::*;

pub mod extensions;
pub mod handlers;
pub mod core;

pub fn plugin(app: &mut App){
    app.add_plugins((core::plugin,));
}
