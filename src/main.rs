use bevy::prelude::*;
use secludia::hello;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(hello::HelloPlugin)
        .run();
}
