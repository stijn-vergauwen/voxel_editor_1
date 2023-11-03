pub mod building;

use bevy::prelude::*;
use flying_camera::{FlyingCameraBundle, FlyingCameraPlugin};

use self::building::CameraBuildingPlugin;
use crate::mouse_interaction::MouseInteraction;

pub struct EditorCameraPlugin;

impl Plugin for EditorCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FlyingCameraPlugin, CameraBuildingPlugin))
            .add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands, mut mouse_interaction: ResMut<MouseInteraction>) {
    let camera_entity = commands
        .spawn((
            Name::new("Flying camera"),
            Camera3dBundle {
                transform: Transform::from_xyz(-6.0, 6.0, 12.0),
                ..Default::default()
            },
            FlyingCameraBundle::default(),
            // CameraInteraction::default(),
        ))
        .id();

    mouse_interaction.set_active_camera(camera_entity);
}
