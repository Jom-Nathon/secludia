
use bevy::prelude::*;

pub struct SettingPlugin;

impl Plugin for SettingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hello_world);
    }
}

fn hello_world() {
    info!("hello world");
}