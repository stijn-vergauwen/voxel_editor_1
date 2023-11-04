pub mod building;
pub mod camera;
mod mouse_interaction;

use bevy::prelude::*;
use building::CameraBuildingPlugin;
use camera::EditorCameraPlugin;
use mouse_interaction::MouseInteractionPlugin;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EditorCameraPlugin,
            MouseInteractionPlugin,
            CameraBuildingPlugin,
        ));
    }
}
