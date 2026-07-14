use bevy::prelude::*;

mod plugins;
use plugins::GenerateMapPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GenerateMapPlugin::GenerateMapPlugin)
        .run();
}
