pub mod building;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;
use flying_camera::{FlyingCameraBundle, FlyingCameraPlugin};

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
// TODO: raycast utilities for interaction. <- doing
// TODO: Select blocks by clicking on them
// TODO: event for when targetblock changes

#[derive(Component)]
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

struct TargetBlock {
    block_entity: Entity,
    normal: Vec3,
    in_position: Vec3,
    out_position: Vec3,
}

impl TargetBlock {
    fn from_raycast(entity: Entity, intersection: RayIntersection) -> Self {
        // TODO: in & out positions don't account for block size
        let point = intersection.point;
        let normal = intersection.normal;

        let in_position = (point - normal / 2.0).round();
        let out_position = (point + normal / 2.0).round();

        Self {
            block_entity: entity,
            normal,
            in_position,
            out_position,
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
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
) {
    for mut camera in cameras.iter_mut() {
        camera.target = cast_ray_to_target_block(&rapier_context, &camera);
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
) -> Option<TargetBlock> {
    let ray = camera.cursor_ray?;

    let intersection = rapier.cast_ray_and_get_normal(
        ray.origin,
        ray.direction,
        camera.ray_distance,
        false,
        QueryFilter::new(),
    );

    intersection.map(|(entity, intersection)| TargetBlock::from_raycast(entity, intersection))
}

#[cfg(test)]
mod tests {
    // use super::*;

    // Idk how to write tests for raycasting from camera since this uses a lot of Bevy components.

    // TODO: create targetBlock
    // TODO: calculate in & out positions
    // TODO: flipped normal should flip in & out
}
