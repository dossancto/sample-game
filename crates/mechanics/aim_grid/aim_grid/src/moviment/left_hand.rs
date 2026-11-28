use aim_grid_gateway::{
    actions::cell_moviments::{
        MoveLeftHandToDown, MoveLeftHandToLeft, MoveLeftHandToRight, MoveLeftHandToUp,
    },
    trigger::MoveLeftHandPosition,
    utils::grid_point::GridPoint,
};
use bevy::prelude::*;
use bevy_enhanced_input::action::events::{Complete, Start};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(move_left)
        .add_observer(move_right)
        .add_observer(move_down)
        .add_observer(move_up);
}

fn move_left(_on: On<Start<MoveLeftHandToLeft>>, mut commands: Commands) {
    commands.trigger(MoveLeftHandPosition {
        move_position: GridPoint::new(-1, 0),
    });
}

fn move_right(_on: On<Start<MoveLeftHandToRight>>, mut commands: Commands) {
    commands.trigger(MoveLeftHandPosition {
        move_position: GridPoint::new(1, 0),
    });
}

fn move_down(_on: On<Start<MoveLeftHandToUp>>, mut commands: Commands) {
    commands.trigger(MoveLeftHandPosition {
        move_position: GridPoint::new(0, -1),
    });
}

fn move_up(_on: On<Start<MoveLeftHandToDown>>, mut commands: Commands) {
    commands.trigger(MoveLeftHandPosition {
        move_position: GridPoint::new(0, 1),
    });
}
