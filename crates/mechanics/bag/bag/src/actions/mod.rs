use bevy::prelude::*;

pub mod put_item_on_bag;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((put_item_on_bag::plugin,));
}
