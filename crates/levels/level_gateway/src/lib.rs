use avian3d::prelude::*;

#[derive(PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    Default,
    Player,
    Sensor,
    Prop
}
