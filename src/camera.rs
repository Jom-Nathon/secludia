//! This example demonstrates how to use the `Camera::viewport_to_world` method.

use bevy::prelude::*;
use bevy::camera_controller::free_camera::{ FreeCamera };

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

#[derive(Component)]
struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 12.0, 18.0).looking_at(Vec3::ZERO, Vec3::Y),
        FreeCamera::default(),
        MainCamera,
    ));
}

// pub fn pan_camera(
//     keys: Res<ButtonInput<KeyCode>>,
//     time: Res<Time>,
//     mut camera: Query<&mut Transform, With<MainCamera>>,
// ) {
//     let Ok(mut transform) = camera.single_mut() else { return };
//     let mut dir = Vec3::ZERO;
//     if keys.pressed(KeyCode::KeyW) { dir.z -= 1.0; }
//     if keys.pressed(KeyCode::KeyS) { dir.z += 1.0; }
//     if keys.pressed(KeyCode::KeyA) { dir.x -= 1.0; }
//     if keys.pressed(KeyCode::KeyD) { dir.x += 1.0; }
//     transform.translation += dir.normalize_or_zero() * 20.0 * time.delta_secs();
// }