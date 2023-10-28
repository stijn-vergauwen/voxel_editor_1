use bevy::prelude::*;

use crate::world::chunk::Chunk;

pub struct SceneLoaderPlugin;

impl Plugin for SceneLoaderPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, ());
    }
}

const SCENE_FILE_PATH: &str = "scenes/test_scene.ron";

const SAVE_KEY: KeyCode = KeyCode::I;
const LOAD_KEY: KeyCode = KeyCode::O;

// TODO: request for saving scene
// TODO: request for loading scene
// TODO: send requests on user input
// TODO: save scene to test file
// TODO: load scene from test file
