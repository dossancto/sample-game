use bevy::prelude::*;

#[derive(Component)]
pub struct CollectableItemStats {
    pub volume: f64,
    pub density: f64,
}

impl CollectableItemStats {
    pub fn mass(&self) -> f64 {
        self.volume * self.density
    }
}
