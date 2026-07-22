
use bevy::{pbr::LightEntity, prelude::*};
use rand::RngExt;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Terrain {
    Land,
    Ocean,
}

#[derive(Debug)]
pub struct NodeData {
    pub position: Vec3,
    pub terrain: Terrain,
}

#[derive(Resource, Debug, Default)]
pub struct WorldGraph {
    pub nodes: Vec<NodeData>,
    pub adjacency: Vec<Vec<usize>>,
}

// #[derive(Component)]
// pub struct NodeRef(pub usize);

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldGraph>()
            .add_systems(Startup, render_seed);
            // .add_systems(Update, wireframe);
    }
}

fn fibonacci_sphere(n: usize) -> Vec<Vec3> {
    let golden_angle = std::f32::consts::PI * (3.0 - 5.0_f32.sqrt());
    (0..n).map(|i| {
        let y = 1.0 - 2.0 * (i as f32 + 0.5) / n as f32;
        let r = (1.0 - y * y).sqrt();
        let theta = golden_angle * i as f32;
        Vec3::new(r * theta.cos(), y, r * theta.sin())
    }).collect()
}

fn render_seed(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let seeds = fibonacci_sphere(1);

    info!("{:?}", seeds);

    let mesh_handle = meshes.add(Sphere::new(0.2));
    let material_handle = materials.add(Color::srgb(0.9, 0.6, 0.2));

    for coord in seeds {
        commands.spawn((
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(material_handle.clone()),
            Transform::from_translation(coord * 10.0),
        ));
    }

    commands.spawn((PointLight::default(), Transform::from_xyz(0.0, 0.0, 0.0)));
}