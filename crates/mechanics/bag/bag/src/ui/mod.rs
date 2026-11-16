use bevy::prelude::*;

pub mod render_bag_size_text;
pub mod item_status;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((render_bag_size_text::plugin, item_status::plugin));
}
