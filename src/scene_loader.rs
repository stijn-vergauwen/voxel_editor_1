use std::{fs::File, io::Write};

use bevy::{prelude::*, tasks::IoTaskPool};

pub struct SceneLoaderPlugin;

impl Plugin for SceneLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, save_test_scene);
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

const SCENE_FILE_PATH: &str = "scenes/test_scene.scn.ron";

fn save_test_scene(world: &mut World) {
    // Scenes can be created from any ECS World.
    // You can either create a new one for the scene or use the current World.
    // For demonstration purposes, we'll create a new one.
    let mut scene_world = World::new();

    // The `TypeRegistry` resource contains information about all registered types (including components).
    // This is used to construct scenes, so we'll want to ensure that our previous type registrations
    // exist in this new scene world as well.
    // To do this, we can simply clone the `AppTypeRegistry` resource.
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    scene_world.insert_resource(type_registry);

    scene_world.spawn((ComponentA { x: 1.0, y: 2.0 }, Transform::IDENTITY));
    scene_world.spawn(ComponentA { x: 3.0, y: 4.0 });
    scene_world.insert_resource(ResourceA { score: 1 });

    // With our sample world ready to go, we can now create our scene:
    let scene = DynamicScene::from_world(&scene_world);

    // Scenes can be serialized like this:
    let type_registry = world.resource::<AppTypeRegistry>();
    let serialized_scene = scene.serialize_ron(type_registry).unwrap();

    // Showing the scene in the console
    info!("{}", serialized_scene);

    // Writing the scene to a new file. Using a task to avoid calling the filesystem APIs in a system
    // as they are blocking
    // This can't work in WASM as there is no filesystem access
    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/{}", SCENE_FILE_PATH))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}
