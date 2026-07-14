use bevy::prelude::*;

// Plugin struct - must be public to be used in main.rs
pub struct GenerateMapPlugin;

// Resource for timing greetings
#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct Unit {
    id: i32,
    name: String,
    health: i32,
    size: i32,
    physical_resistance: i32,
    magic_resistance: i32,
    strength: i32,
    agility: i32,
    intelligence: i32,
    attack: i32,
    defense: i32,
    speed: i32,
    equip_load: i32,
    fatigue: i32,
    age: i32,
    tag: Vec<String>,
}

struct God {
    id: i32,
    name: String,
    health: i32,
    size: i32,
    physical_resistance: i32,
    magic_resistance: i32,
    strength: i32,
    agility: i32,
    intelligence: i32,
    attack: i32,
    defense: i32,
    speed: i32,
    equip_load: i32,
    fatigue: i32,
    age: i32,
    tag: Vec<String>,
}

struct Map {
    id_to_entity: HashMap<
}

impl Plugin for GenerateMapPlugin {
    fn build(&self, app: &mut App) {
        print!("hello world")
        command.spawn(bundle: (
            SpatialBundle::default();
        ))
    }
}

