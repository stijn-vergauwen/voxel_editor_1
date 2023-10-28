use std::fs;

use bevy::prelude::*;

use crate::world::chunk::Chunk;

pub struct SceneLoaderPlugin;

impl Plugin for SceneLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnSaveSceneRequest>()
            .add_event::<OnLoadSceneRequest>()
            .add_systems(
                Update,
                (send_requests_on_keyboard_input, handle_save_requests),
            );
    }
}

const FILE_PATH_TO_SAVES: &str = "assets/scenes/test_save.ron";

const SAVE_KEY: KeyCode = KeyCode::I;
const LOAD_KEY: KeyCode = KeyCode::O;

// TODO: save scene to test file <- doing
// TODO: load scene from test file

#[derive(Event)]
struct OnSaveSceneRequest;

#[derive(Event)]
struct OnLoadSceneRequest;

fn send_requests_on_keyboard_input(
    input: Res<Input<KeyCode>>,
    mut on_save_request: EventWriter<OnSaveSceneRequest>,
    mut on_load_request: EventWriter<OnLoadSceneRequest>,
) {
    if input.just_pressed(SAVE_KEY) {
        println!("Request save");
        on_save_request.send(OnSaveSceneRequest);
    }

    if input.just_pressed(LOAD_KEY) {
        println!("Request load");
        on_load_request.send(OnLoadSceneRequest);
    }
}

fn handle_save_requests(chunks: Query<&Chunk>, on_save_request: EventReader<OnSaveSceneRequest>) {
    if !on_save_request.is_empty() {
        save_chunk_to_file(chunks.single(), FILE_PATH_TO_SAVES);
    }
}

fn save_chunk_to_file(chunk: &Chunk, path: &str) {
    if let Ok(serialized) = ron::to_string(&chunk) {
        let result = fs::write(path, serialized);
        
        if let Err(err) = result {
            println!("Error while saving data to file! error: {}", err)
        };
    }
}
