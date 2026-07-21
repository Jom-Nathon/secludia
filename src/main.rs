use bevy::prelude::*;
use bevy::camera_controller::free_camera::{ FreeCameraPlugin };

use secludia::map;
use secludia::camera;
use secludia::unit::{self, UnitPlugin};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FreeCameraPlugin)
        .add_plugins(UnitPlugin)
        .add_systems(Startup, (map::render_map, camera::spawn_camera))
        .run();
}
