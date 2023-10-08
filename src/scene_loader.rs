use std::{fs::File, io::Write};

use bevy::{prelude::*, tasks::IoTaskPool};

pub struct SceneLoaderPlugin;

impl Plugin for SceneLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ComponentA>()
            .register_type::<ResourceA>()
            .add_systems(Startup, save_test_scene);
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)] // this tells the reflect derive to also reflect component behaviors
struct ComponentA {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct ResourceA {
    pub score: u32,
}

impl ResourceA {
    fn new(score: u32) -> Self {
        Self { score }
    }
}

const SCENE_FILE_PATH: &str = "scenes/test_scene.scn.ron";

fn save_test_scene(world: &mut World) {
    let mut scene_world = create_empty_world(clone_type_registry(&world));

    scene_world.spawn((ComponentA { x: 1.0, y: 2.0 }, Transform::IDENTITY));
    scene_world.spawn(ComponentA { x: 3.0, y: 4.0 });
    scene_world.insert_resource(ResourceA::new(3));

    save_world_data(String::from(SCENE_FILE_PATH), world);
}

fn create_empty_world(type_registry: AppTypeRegistry) -> World {
    let mut world = World::new();
    world.insert_resource(type_registry);
    world
}

fn clone_type_registry(world: &World) -> AppTypeRegistry {
    world.resource::<AppTypeRegistry>().clone()
}

fn serialize_world_data(world: &World) -> Option<String> {
    let scene = DynamicScene::from_world(&world);
    let type_registry = world.resource::<AppTypeRegistry>();

    scene.serialize_ron(type_registry).ok()
}

fn write_data_to_file(file_path: String, serialized_data: String) {
    IoTaskPool::get()
        .spawn(async move {
            File::create(format!("assets/{}", file_path))
                .and_then(|mut file| file.write(serialized_data.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}

fn save_world_data(file_path: String, world: &World) {
    if let Some(serialized_data) = serialize_world_data(world) {
        write_data_to_file(file_path, serialized_data);
    }
}
