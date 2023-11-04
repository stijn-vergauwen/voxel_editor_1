use std::fs;

use bevy::prelude::*;
use ron::Error;

use crate::world::chunk::Chunk;

pub struct SceneLoaderPlugin;

impl Plugin for SceneLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnSaveSceneRequest>()
            .add_event::<OnLoadSceneRequest>()
            .add_systems(
                Update,
                (
                    send_requests_on_keyboard_input,
                    handle_save_requests,
                    handle_load_requests,
                ),
            );
    }
}

const FILE_PATH_TO_SAVES: &str = "assets/scenes/test_save.ron";

const SAVE_KEY: KeyCode = KeyCode::I;
const LOAD_KEY: KeyCode = KeyCode::O;

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
        if let Err(error) = save_chunk_to_file(chunks.single(), FILE_PATH_TO_SAVES) {
            println!("Error while saving chunk: {:?}", error);
        }
    }
}

fn handle_load_requests(
    mut commands: Commands,
    chunks: Query<Entity, With<Chunk>>,
    on_load_request: EventReader<OnLoadSceneRequest>,
) {
    if !on_load_request.is_empty() {
        let chunk_entity = chunks.single();

        match load_chunk_from_file(FILE_PATH_TO_SAVES) {
            Ok(mut new_chunk) => {
                new_chunk.set_changed();
                commands.entity(chunk_entity).insert(new_chunk);
            }
            Err(error) => {
                println!("Error while loading chunk: {:?}", error);
            }
        }
    }
}

fn save_chunk_to_file(chunk: &Chunk, path: &str) -> Result<(), Error> {
    let serialized = ron::to_string(&chunk)?;
    Ok(fs::write(path, serialized)?)
}

fn load_chunk_from_file(path: &str) -> Result<Chunk, Error> {
    let file = fs::read_to_string(path)?;
    Ok(ron::from_str::<Chunk>(&file)?)
}
