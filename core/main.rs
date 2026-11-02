use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, third_parties::plugin, level::plugin, player::plugin))
        .run();
}
