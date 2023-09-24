mod camera;
mod world;

use bevy::{prelude::*, window};
use bevy_rapier3d::prelude::*;
use camera::EditorCameraPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
            EditorCameraPlugin,
        ))
        .add_systems(Startup, spawn_light)
        .add_systems(Update, window::close_on_esc)
        .run();
}
fn spawn_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
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
    });
}
