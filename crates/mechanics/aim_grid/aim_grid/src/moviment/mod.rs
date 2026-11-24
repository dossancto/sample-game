use bevy::prelude::*;

pub mod controll;
pub mod left_hand;
pub mod right_hand;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((left_hand::plugin, right_hand::plugin, controll::plugin));
}
