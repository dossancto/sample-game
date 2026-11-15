use avian3d::prelude::*;
use bevy::prelude::*;

#[derive(PhysicsLayer, Default)]
pub enum CollisionLayer {
    #[default]
    Default,
    Player,
    Sensor,
    Prop
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Screen {
    #[default]
    Splash,
    Title,
    Loading,
    Gameplay,
}
