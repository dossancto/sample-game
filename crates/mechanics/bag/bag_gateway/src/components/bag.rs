use bevy::prelude::*;

use crate::components::collectable_item::CollectableItemStats;

#[derive(Component)]
pub struct Bag {
    pub max_volume: f64,
    pub max_mass: f64,

    pub current_volume: f64,
    pub current_mass: f64,
}

impl Bag {
    pub fn new(max_volume: f64, max_mass: f64) -> Bag {
        Bag {
            max_volume: max_volume,
            max_mass: max_mass,

            current_volume: 0f64,
            current_mass: 0f64,
        }
    }

    pub fn can_add_volume(&self, volume: f64) -> bool {
        let new_volume = self.current_volume + volume;
        new_volume <= self.max_volume
    }

    pub fn can_add_mass(&self, mass: f64) -> bool {
        let new_mass = self.current_mass + mass;
        new_mass <= self.max_mass
    }

    pub fn add(&mut self, volume: f64, mass: f64)  {
        let new_volume = self.current_volume + volume;
        let new_mass = self.current_mass + mass;

        if new_volume > self.max_volume || new_mass > self.max_mass {
            return ;
        }

        self.current_mass = new_mass;
        self.current_volume = new_volume;

    }

    pub fn add_collectable_item(&mut self, item: &CollectableItemStats) {
        self.add(item.volume, item.mass());
    }
}
