use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((TrenchBroomPlugins(
        TrenchBroomConfig::new("my-blend").default_solid_scene_hooks(|| {
            SceneHooks::new()
                .convex_collider()
                .smooth_by_default_angle()
        }),
    ),));
}
