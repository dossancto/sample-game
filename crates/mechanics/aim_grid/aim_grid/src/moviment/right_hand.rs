use aim_grid_gateway::{
    actions::cell_moviments::{
        MoveRightHandToDown, MoveRightHandToRight, MoveRightHandToLeft, MoveRightHandToUp,
    },
    trigger::MoveRightHandPosition,
    utils::grid_point::GridPoint,
};
use bevy::prelude::*;
use bevy_enhanced_input::action::events::Complete;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(move_left)
        .add_observer(move_right)
        .add_observer(move_down)
        .add_observer(move_up);
}

fn move_left(_on: On<Complete<MoveRightHandToLeft>>, mut commands: Commands) {
    commands.trigger(MoveRightHandPosition {
        move_position: GridPoint::new(-1, 0),
    });
}

fn move_right(_on: On<Complete<MoveRightHandToRight>>, mut commands: Commands) {
    commands.trigger(MoveRightHandPosition {
        move_position: GridPoint::new(1, 0),
    });
}

fn move_down(_on: On<Complete<MoveRightHandToUp>>, mut commands: Commands) {
    commands.trigger(MoveRightHandPosition {
        move_position: GridPoint::new(0, -1),
    });
}

fn move_up(_on: On<Complete<MoveRightHandToDown>>, mut commands: Commands) {
    commands.trigger(MoveRightHandPosition {
        move_position: GridPoint::new(0, 1),
    });
}

