use bevy::prelude::*;

use crate::utils::grid_point::GridPoint;

#[derive(Component)]
pub struct RightHandCurrentCell {
    pub current_cell: GridPoint,
}

#[derive(Component)]
pub struct LeftHandCurrentCell {
    pub current_cell: GridPoint,
}

#[derive(Bundle)]
pub struct HandStatusBundle {
    pub left_hand_current_cell: LeftHandCurrentCell,
    pub right_hand_current_cell: RightHandCurrentCell,
}

impl Default for HandStatusBundle {
    fn default() -> Self {
        Self {
            left_hand_current_cell: LeftHandCurrentCell {
                current_cell: GridPoint::new(0, 0),
            },
            right_hand_current_cell: RightHandCurrentCell {
                current_cell: GridPoint::new(0, 0),
            },
        }
    }
}
