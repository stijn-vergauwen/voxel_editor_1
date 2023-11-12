pub mod game_systems;
pub mod newtypes;
pub mod player;
mod world;

use bevy::{prelude::*, window};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use game_systems::GameSystemsPlugin;
use player::PlayerPlugin;
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

Ok I'm calling this project done here, it's less than what I had in these plans but I made good progress towards them!
Some things I struggled with in this project:
- UI, I still feel a lot of friction when making menu's or buttons and stuff, and sliders or text inputs have to be manually made afaik
- Events, just not quite used to thinking in events, learned a lot though.
- Input, tied to events, user input is also something I don't have standard solutions for yet, and this project needed quite a bit of it
- Saving & Loading, this derailed me a bit since I didn't know anything about reflection, for now I made my own workaround.

On to the next version!

*/

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new(),
            GameSystemsPlugin,
            WorldPlugin,
            PlayerPlugin,
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
