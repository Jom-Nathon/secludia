
use bevy::prelude::*;

pub struct BattlefieldPlugin;

impl Plugin for BattlefieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hello_world);
    }
}

fn hello_world() {
    info!("hello world");
}