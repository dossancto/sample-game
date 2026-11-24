use aim_grid_gateway::{
    components::hand_status::{LeftHandCurrentCell, RightHandCurrentCell},
    trigger::{MoveLeftHandPosition, MoveRightHandPosition},
};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(handle_grid_left_hand_move);
    app.add_observer(handle_grid_right_hand_move);
}

fn handle_grid_left_hand_move(
    on: On<MoveLeftHandPosition>,
    mut left_hand_current_cell: Single<&mut LeftHandCurrentCell>,
) {
    left_hand_current_cell.current_cell =
        left_hand_current_cell.current_cell.add(&on.move_position);
}

fn handle_grid_right_hand_move(
    on: On<MoveRightHandPosition>,
    mut right_hand_current_cell: Single<&mut RightHandCurrentCell>,
) {
    right_hand_current_cell.current_cell =
        right_hand_current_cell.current_cell.add(&on.move_position);
}
