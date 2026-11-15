use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_ahoy::AhoyPlugins;
use bevy_enhanced_input::prelude::*;

pub mod packages;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        EnhancedInputPlugin,
        PhysicsPlugins::default(),
        AhoyPlugins::default(),
        packages::plugin,
    ));
}
