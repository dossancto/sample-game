use bevy::prelude::*;

#[derive(Component, Default)]
pub struct MoneyBag {
    pub value: f64,
}

impl MoneyBag {
    pub fn add_money(&mut self, value: f64) {
        self.value += value;
    }
}

#[derive(Component)]
pub struct CollectableMoney {
    pub value: f64,
}

#[derive(Event)]
pub struct MoneyCollected {
    pub value: f64,
}
