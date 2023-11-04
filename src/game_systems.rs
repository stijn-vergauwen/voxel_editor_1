pub mod color_library;
mod scene_loader;

use bevy::prelude::*;
use color_library::ColorLibraryPlugin;
use scene_loader::SceneLoaderPlugin;

pub struct GameSystemsPlugin;

impl Plugin for GameSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ColorLibraryPlugin, SceneLoaderPlugin));
    }
}
