use std::{fs::File, io::Write};

use bevy::{prelude::*, tasks::IoTaskPool};

use crate::world::chunk::Chunk;

pub struct SceneLoaderPlugin;

impl Plugin for SceneLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_save_input, handle_load_input));
    }
}

const SCENE_FILE_PATH: &str = "scenes/test_scene.scn.ron";

const SAVE_KEY: KeyCode = KeyCode::I;
const LOAD_KEY: KeyCode = KeyCode::O;

fn handle_save_input(world: &mut World) {
    let input = world.resource::<Input<KeyCode>>().clone();

    if input.just_pressed(SAVE_KEY) {
        save_test_scene(world);
    }
}

fn handle_load_input(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    chunk: Query<Entity, With<Chunk>>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(LOAD_KEY) {
        load_test_scene(&mut commands, &asset_server, chunk.single());
    }
}

fn save_test_scene(world: &mut World) {
    let mut scene_world = create_empty_world(clone_type_registry(&world));

    for chunk in world.query::<&Chunk>().iter(world) {
        scene_world.spawn(chunk.clone());
    }

    save_world_data(String::from(SCENE_FILE_PATH), &scene_world);
}

fn load_test_scene(commands: &mut Commands, asset_server: &AssetServer, chunk_entity: Entity) {
    // TODO: how do I transform a dynamic scene handle to chunk data? do I need to wrap the original chunk in a dynamic scene too?

    let loaded_chunk: Handle<DynamicScene> = asset_server.load(SCENE_FILE_PATH);
    println!("{:#?}", loaded_chunk);

    commands.entity(chunk_entity).despawn_recursive();

    // I'm getting the message "SceneInstance is not registered in the TypeRegistry" when opening SceneInstance in bevy inspector.
    // One thing to try out: make a separate struct ChunkData to represent the data in a way that's easier to serialize (flat array instead of nested array). Then switch between these

    // TODO: find out how to load a dynamic scene.
    commands.spawn(DynamicSceneBundle {
        scene: loaded_chunk,
        ..default()
    });
}

fn create_empty_world(type_registry: AppTypeRegistry) -> World {
    let mut world = World::new();
    world.insert_resource(type_registry);
    world
}

fn clone_type_registry(world: &World) -> AppTypeRegistry {
    world.resource::<AppTypeRegistry>().clone()
}

fn save_world_data(file_path: String, world: &World) {
    if let Some(serialized_data) = serialize_world_data(world) {
        write_data_to_file(file_path, serialized_data);
    }
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
