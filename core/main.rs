use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, level::plugin))
        .run();
}
