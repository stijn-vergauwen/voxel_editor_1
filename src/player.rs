pub mod camera;
mod mouse_interaction;
pub mod editor_modes;

use bevy::prelude::*;
use camera::EditorCameraPlugin;
use mouse_interaction::MouseInteractionPlugin;

use self::editor_modes::EditorModesPlugin;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EditorCameraPlugin,
            MouseInteractionPlugin,
            EditorModesPlugin,
        ));
    }
}
