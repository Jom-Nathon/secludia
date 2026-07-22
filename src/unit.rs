
use bevy::prelude::*;

use rand::RngExt;

#[derive(Component)]
struct Unit;

#[derive(Component)]
struct Health(pub f32);

#[derive(Component)]
struct Speed(pub f32);

#[derive(Component)]
struct Attack(pub i32);

pub struct UnitPlugin;


impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_units);
    }
}

fn spawn_units(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, keys: Res<ButtonInput<KeyCode>>) {

    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    let mut rng = rand::rng();

    let mesh = meshes.add(Capsule3d::new(rng.random::<f32>(), rng.random::<f32>()));
    let material = materials.add(Color::srgb(rng.random::<f32>(), rng.random::<f32>(), rng.random::<f32>()));

    let x = -5.0 + rng.random::<f32>() * 10.0;
    let y = rng.random::<f32>();
    let z = -5.0 + rng.random::<f32>() * 10.0;

    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(x , y, z),
        Unit,
        Health(100.0),
        Speed(3.0),
        Attack(rng.random::<i32>())
    ));

    info!("Spawn unit at coord x: {}, y: {}, z: {}", x,y,z)

}