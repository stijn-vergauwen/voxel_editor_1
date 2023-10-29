mod camera;
mod color_library;
mod mouse_interaction;
pub mod newtypes;
mod scene_loader;
mod world;

use bevy::{prelude::*, window};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use camera::EditorCameraPlugin;
use color_library::ColorLibraryPlugin;
use mouse_interaction::MouseInteractionPlugin;
use scene_loader::SceneLoaderPlugin;
use world::WorldPlugin;

/* Standards to work towards in this project:

    - Build this project using TDD. Write tests first, then functionality.
    - System functions should mainly pass data, decouple querying from calculations.
    - Use events for everything that doesn't happen every frame. (interactions etc.)

*/

/* Project plans:

- General:
    - Color mixer & picker stuff.
    - Screenshots, for thumbnail image & just for taking pictures and saving to file.

- Editor tools:
    - Build with click.
    - Drag to build multiple blocks.
    - Block selections, to move or delete the selection.

- Data:
    - Save & load all built blocks to files.
    - Overview menu of these files (open, delete, rename etc.).

*/

// TODO: block world interaction when ui is clicked
// TODO: a build & a select mode
// TODO: track selected blocks when in select mode
// TODO: click a color to change selection to that color
// TODO: delete selection of blocks with del key
// TODO: drag build to build multiple blocks at once
// TODO: preview the blocks before they're built

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new(),
            EditorCameraPlugin,
            WorldPlugin,
            ColorLibraryPlugin,
            SceneLoaderPlugin,
            MouseInteractionPlugin,
        ))
        .add_systems(Startup, spawn_light)
        .add_systems(Update, window::close_on_esc)
        .run();
}

fn spawn_light(mut commands: Commands) {
    commands.spawn((
        Name::new("Directional light"),
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 10_000.0,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 20.0, 0.0),
                rotation: Quat::from_rotation_x(-45f32.to_radians()),
                ..default()
            },
            ..default()
        },
    ));
}
