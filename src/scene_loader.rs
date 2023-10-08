use std::{fs::File, io::Write};

use bevy::{prelude::*, tasks::IoTaskPool};

use crate::world::chunk::Chunk;

pub struct SceneLoaderPlugin;

impl Plugin for SceneLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, save_test_scene_on_keypress);
    }
}

const SCENE_FILE_PATH: &str = "scenes/test_scene.scn.ron";

fn save_test_scene_on_keypress(world: &mut World) {
    if !world.resource::<Input<KeyCode>>().just_pressed(KeyCode::S) {
        return;
    }

    let mut scene_world = create_empty_world(clone_type_registry(&world));

    for chunk in world.query::<&Chunk>().iter(world) {
        scene_world.spawn(*chunk);
    }

    println!(
        "test scene has {} entities",
        scene_world.entities().total_count()
    );

    save_world_data(String::from(SCENE_FILE_PATH), &scene_world);
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
