use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerMoving;

#[derive(Component)]
pub struct PlayerMovimentControll {
    pub speed: f32,
}
