
use bevy::prelude::*;

#[derive(Component)]
struct Node;

#[derive(Component)]
struct Connection;

#[derive(Component)]
struct PlateGroup;

#[derive(Component)]
struct PlateMovementVector;

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (render_map).chain());
    }
}

fn render_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1., 1.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Node,
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