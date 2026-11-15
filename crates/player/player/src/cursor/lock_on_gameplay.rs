use bevy::{prelude::*, window::{CursorGrabMode, CursorOptions}};
use level_gateway::Screen;

pub(super) fn plugin(app: &mut App){
    app.add_systems(OnEnter(Screen::Gameplay), lock_cursor);
}

fn lock_cursor(
    mut cursor: Single<&mut CursorOptions>) {
    cursor.visible = false;
    cursor.grab_mode = CursorGrabMode::Locked;
}
