pub mod building;
mod target;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use flying_camera::{FlyingCameraBundle, FlyingCameraPlugin};

use crate::{mouse_interaction::MouseInteraction, newtypes::direction::Direction};

use self::{
    building::CameraBuildingPlugin,
    target::{CameraTargetPlugin, OnTargetBlockChanged, TargetBlock},
};

pub struct EditorCameraPlugin;

impl Plugin for EditorCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FlyingCameraPlugin, CameraBuildingPlugin, CameraTargetPlugin))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, update_camera_target);
    }
}

struct RayHit {
    point: Vec3,
    normal: Direction,
}

impl RayHit {
    #[allow(unused)]
    fn new(point: Vec3, normal: Direction) -> Self {
        Self { point, normal }
    }
}

impl From<RayIntersection> for RayHit {
    fn from(value: RayIntersection) -> Self {
        Self {
            point: value.point,
            normal: Direction::from_vector(value.normal),
        }
    }
}

#[derive(Component, Debug)]
struct CameraInteraction {
    target: Option<TargetBlock>,
}

impl Default for CameraInteraction {
    fn default() -> Self {
        Self {
            target: None,
        }
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
            CameraInteraction::default(),
        ))
        .id();

    mouse_interaction.set_active_camera(camera_entity);
}

fn update_camera_target(
    mut cameras: Query<&mut CameraInteraction>,
    mut on_target_changed: EventReader<OnTargetBlockChanged>,
) {
    for event in on_target_changed.iter() {
        if let Ok(mut camera) = cameras.get_mut(event.camera) {
            camera.target = event.new_target;
        }
    }
}


#[cfg(test)]
mod tests {
    use bevy_rapier3d::rapier::prelude::FeatureId;

    use crate::newtypes::direction::Direction;

    use super::*;

    #[test]
    fn can_create_ray_hit() {
        let ray_hit = RayHit::new(Vec3::new(2.0, 2.0, 2.0), Direction::Y);

        assert_eq!(ray_hit.point, Vec3::new(2.0, 2.0, 2.0));
        assert_eq!(ray_hit.normal, Direction::Y);
    }

    #[test]
    fn can_create_ray_hit_from_ray_intersection() {
        let intersection = RayIntersection {
            feature: FeatureId::Face(0),
            normal: Vec3::X,
            point: Vec3::new(1.5, 0.0, 1.8),
            toi: 1.0,
        };

        let ray_hit = RayHit::from(intersection);

        assert_eq!(ray_hit.point, Vec3::new(1.5, 0.0, 1.8));
        assert_eq!(ray_hit.normal, Direction::X);
    }
}
