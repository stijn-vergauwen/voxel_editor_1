use bevy::prelude::*;
use flying_camera::{FlyingCameraBundle, FlyingCameraPlugin};

pub struct EditorCameraPlugin;

impl Plugin for EditorCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FlyingCameraPlugin)
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-6.0, 6.0, 12.0),
            ..Default::default()
        },
        FlyingCameraBundle::default(),
    ));
}
