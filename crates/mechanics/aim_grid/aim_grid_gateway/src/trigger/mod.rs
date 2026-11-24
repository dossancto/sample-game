use bevy::prelude::*;

use crate::utils::grid_point::GridPoint;

#[derive(Event)]
pub struct MoveLeftHandPosition {
    pub move_position: GridPoint,
}

#[derive(Event)]
pub struct MoveRightHandPosition {
    pub move_position: GridPoint,
}
