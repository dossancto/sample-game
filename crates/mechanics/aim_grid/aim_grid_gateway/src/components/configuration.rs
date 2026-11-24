use bevy::prelude::*;

#[derive(Component)]
pub struct GameCellGrid {
    pub width: u8,
    pub height: u8,
}

impl Default for GameCellGrid {
    fn default() -> Self {
        Self {
            width: 4,
            height: 3,
        }
    }
}
