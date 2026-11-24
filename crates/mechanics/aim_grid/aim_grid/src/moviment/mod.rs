use bevy::prelude::*;

pub mod controll;
pub mod left_hand;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((left_hand::plugin, controll::plugin));
}
