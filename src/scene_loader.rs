use bevy::prelude::*;

pub struct SceneLoaderPlugin;

impl Plugin for SceneLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnSaveSceneRequest>()
            .add_event::<OnLoadSceneRequest>()
            .add_systems(Update, send_requests_on_keyboard_input);
    }
}

const SCENE_FILE_PATH: &str = "scenes/test_scene.ron";

const SAVE_KEY: KeyCode = KeyCode::I;
const LOAD_KEY: KeyCode = KeyCode::O;

// TODO: save scene to test file
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
