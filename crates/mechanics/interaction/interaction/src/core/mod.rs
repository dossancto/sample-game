use bevy::prelude::*;

pub mod detect_can_interact;

pub(super) fn plugin(app: &mut App){
    app.add_plugins((detect_can_interact::plugin,));
}
