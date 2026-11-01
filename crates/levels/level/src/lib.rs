use bevy::{prelude::*, world_serialization::WorldInstanceReady};
use bevy_skein::SkeinPlugin;

pub fn plugin(app: &mut App) {
    app.add_plugins((SkeinPlugin::default(),));

    app.add_observer(
        |ready: On<WorldInstanceReady>,
         children: Query<&Children>,
         characters: Query<&Character>| {
            for entity in children.iter_descendants(ready.entity) {
                let Ok(character) = characters.get(entity) else {
                    continue;
                };
                info!(?character);
            }
        },
    );

    app.add_systems(Startup, startup.spawn());
}

#[derive(Component, Default, Reflect, Debug)]
#[reflect(Component, Default)]
#[type_path = "api"]
struct Character {
    name: String,
}

fn startup() -> impl Scene {
    bsn! {
        WorldAssetRoot("demo.gltf#Scene0")
    }
}
