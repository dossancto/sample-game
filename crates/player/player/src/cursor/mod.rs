use bevy::prelude::*;

mod lock_on_gameplay;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((lock_on_gameplay::plugin,));
}
