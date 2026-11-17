use bevy::prelude::*;

#[derive(Component)]
pub struct MoneyBag {
    pub value: u32,
}

#[derive(Component)]
pub struct CollectableMoney {
    pub value: u32,
}
