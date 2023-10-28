pub mod building;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;
use flying_camera::{FlyingCameraBundle, FlyingCameraPlugin};

use crate::world::WorldSettings;

use self::building::CameraBuildingPlugin;

pub struct EditorCameraPlugin;

impl Plugin for EditorCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FlyingCameraPlugin, CameraBuildingPlugin))
            .add_systems(Startup, spawn_camera)
            .add_systems(
                Update,
                (
                    update_cursor_ray,
                    update_interaction_target,
                    draw_target_block_gizmos,
                ),
            );
    }
}

// TODO: make from the depths style camera, maybe replace current with it.
// TODO: Select blocks by clicking on them
// TODO: event for when targetblock changes

struct RayHit {
    point: Vec3,
    normal: Vec3,
    distance: f32,
}

impl RayHit {
    fn new(point: Vec3, normal: Vec3, distance: f32) -> Self {
        Self {
            point,
            normal,
            distance,
        }
    }
}

impl From<RayIntersection> for RayHit {
    fn from(value: RayIntersection) -> Self {
        Self {
            point: value.point,
            normal: value.normal,
            distance: value.toi,
        }
    }
}

#[derive(Component, Debug)]
struct CameraInteraction {
    ray_distance: f32,
    cursor_ray: Option<Ray>,
    target: Option<TargetBlock>,
}

impl Default for CameraInteraction {
    fn default() -> Self {
        Self {
            cursor_ray: None,
            target: None,
            ray_distance: 20.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct TargetBlock {
    normal: Vec3,
    in_position: Vec3,
    out_position: Vec3,
}

impl TargetBlock {
    fn from_raycast(hit: RayHit, block_scale: f32) -> Self {
        let point = hit.point;
        let normal = hit.normal;

        let in_position = ((point / block_scale) - normal / 2.0).round() * block_scale;
        let out_position = ((point / block_scale) + normal / 2.0).round() * block_scale;

        Self {
            normal,
            in_position,
            out_position,
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle {
            transform: Transform::from_xyz(-6.0, 6.0, 12.0),
            ..Default::default()
        },
        FlyingCameraBundle::default(),
        CameraInteraction::default(),
    ));
}

fn update_cursor_ray(
    mut cameras: Query<(&mut CameraInteraction, &Camera, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = window.get_single() {
        for (mut interaction, camera, transform) in cameras.iter_mut() {
            interaction.cursor_ray = get_cursor_as_ray(camera, transform, window);
        }
    }
}

fn update_interaction_target(
    rapier_context: Res<RapierContext>,
    mut cameras: Query<&mut CameraInteraction>,
    world_settings: Res<WorldSettings>,
) {
    for mut camera in cameras.iter_mut() {
        camera.target =
            cast_ray_to_target_block(&rapier_context, &camera, world_settings.block_scale());
    }
}

fn draw_target_block_gizmos(cameras: Query<&CameraInteraction>, mut gizmos: Gizmos) {
    for camera in cameras.iter() {
        if let Some(target) = &camera.target {
            gizmos.cuboid(
                Transform::from_translation(target.in_position),
                Color::WHITE,
            );
            gizmos.cuboid(
                Transform::from_translation(target.out_position),
                Color::CYAN,
            );
            gizmos.ray(target.in_position, target.normal, Color::BLUE);
        }
    }
}

// Utilities

fn get_cursor_as_ray(
    camera: &Camera,
    global_transform: &GlobalTransform,
    window: &Window,
) -> Option<Ray> {
    camera.viewport_to_world(global_transform, window.cursor_position()?)
}

fn cast_ray_to_target_block(
    rapier: &RapierContext,
    camera: &CameraInteraction,
    block_scale: f32,
) -> Option<TargetBlock> {
    let ray = camera.cursor_ray?;

    let intersection = rapier.cast_ray_and_get_normal(
        ray.origin,
        ray.direction,
        camera.ray_distance,
        false,
        QueryFilter::new(),
    );

    intersection
        .map(|(_, intersection)| TargetBlock::from_raycast(RayHit::from(intersection), block_scale))
}

#[cfg(test)]
mod tests {
    use bevy_rapier3d::rapier::prelude::FeatureId;

    use super::*;

    // TODO: replace in & out positions with coordinates, this data points to block coordinates not just position.

    #[test]
    fn can_create_ray_hit() {
        let ray_hit = RayHit::new(Vec3::new(2.0, 2.0, 2.0), Vec3::Y, 3.0);

        assert_eq!(ray_hit.point, Vec3::new(2.0, 2.0, 2.0));
        assert_eq!(ray_hit.normal, Vec3::Y);
        assert_eq!(ray_hit.distance, 3.0);
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
        assert_eq!(ray_hit.normal, Vec3::X);
        assert_eq!(ray_hit.distance, 1.0);
    }

    #[test]
    fn can_create_target_block_from_raycast() {
        let ray_hit = RayHit::new(Vec3::new(1.0, 0.0, 1.8), Vec3::X, 0.0);
        let block_scale = 1.0;

        let target_block = TargetBlock::from_raycast(ray_hit, block_scale);

        assert_eq!(target_block.normal, Vec3::X);
    }

    #[test]
    fn target_block_calculates_in_position() {
        let block_scale = 1.0;
        let target_block = TargetBlock::from_raycast(
            RayHit::new(Vec3::new(1.5, 0.0, 1.8), Vec3::X, 2.0),
            block_scale,
        );

        assert_eq!(target_block.in_position, Vec3::new(1.0, 0.0, 2.0));

        let target_block = TargetBlock::from_raycast(
            RayHit::new(Vec3::new(7.8, 3.4, 7.2), Vec3::Y, 1.0),
            block_scale,
        );

        assert_eq!(target_block.in_position, Vec3::new(8.0, 3.0, 7.0));
    }

    #[test]
    fn target_block_calculates_out_position() {
        let block_scale = 1.0;
        let target_block = TargetBlock::from_raycast(
            RayHit::new(Vec3::new(3.5, 0.0, 2.8), Vec3::X, 2.0),
            block_scale,
        );

        assert_eq!(target_block.out_position, Vec3::new(4.0, 0.0, 3.0));

        let target_block = TargetBlock::from_raycast(
            RayHit::new(Vec3::new(5.8, 2.4, 3.2), Vec3::Y, 1.0),
            block_scale,
        );

        assert_eq!(target_block.out_position, Vec3::new(6.0, 3.0, 3.0));
    }

    #[test]
    fn target_block_accounts_for_block_scale() {
        let ray_hit = RayHit::new(Vec3::new(3.0, 0.0, 0.0), Vec3::X, 1.0);
        let block_scale = 2.0;

        let target_block = TargetBlock::from_raycast(ray_hit, block_scale);

        assert_eq!(target_block.in_position, Vec3::new(2.0, 0.0, 0.0));
        assert_eq!(target_block.out_position, Vec3::new(4.0, 0.0, 0.0));

        let ray_hit = RayHit::new(Vec3::new(4.0, 0.0, 2.0), Vec3::X, 1.0);
        let block_scale = 3.5;

        let target_block = TargetBlock::from_raycast(ray_hit, block_scale);

        assert_eq!(target_block.in_position, Vec3::new(3.5, 0.0, 3.5));
        assert_eq!(target_block.out_position, Vec3::new(7.0, 0.0, 3.5));
    }
}
