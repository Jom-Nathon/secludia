
use bevy::prelude::*;
use bevy::camera_controller::free_camera::{ FreeCameraPlugin };

mod setup;
mod setting;
mod camera;
mod worldmap;
mod battlefield;
mod unit;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            setup::SettingPlugin,
            setting::SettingPlugin,
            camera::CameraPlugin,
            worldmap::WorldMapPlugin,
            battlefield::BattlefieldPlugin,
            unit::UnitPlugin,

            FreeCameraPlugin,
        ));
    }
}