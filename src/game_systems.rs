pub mod color_library;
mod editor_modes;
mod scene_loader;

use bevy::prelude::*;
use color_library::ColorLibraryPlugin;
use editor_modes::EditorModesPlugin;
use scene_loader::SceneLoaderPlugin;

pub struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ColorLibraryPlugin, EditorModesPlugin, SceneLoaderPlugin));
    }
}
