//! This example demonstrates how to use the `Camera::viewport_to_world` method.

use bevy::prelude::*;

#[derive(Component)]
struct Ground;

#[derive(Component)]
pub struct MainCamera;

pub fn render_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Ground,
    ));

    // // light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// pub fn generate_continent(large_landmass: u32, small) {
//     command.spawn
// }