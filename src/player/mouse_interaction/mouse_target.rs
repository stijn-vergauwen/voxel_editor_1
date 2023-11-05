use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

use super::MouseInteraction;
use crate::{
    newtypes::{coordinate::Coordinate, direction::Direction},
    world::WorldSettings,
};

pub struct MouseTargetPlugin;

impl Plugin for MouseTargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnMouseTargetChanged>().add_systems(
            Update,
            (
                update_interaction_ray,
                update_mouse_target,
                draw_target_block_gizmos,
            ),
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseTarget {
    pub point: Vec3,
    pub normal: Direction,
    pub in_coord: Coordinate,
    pub out_coord: Coordinate,
}

impl MouseTarget {
    fn from_raycast(intersection: RayIntersection, block_scale: f32) -> Self {
        let point = intersection.point;
        let normal = intersection.normal;

        let in_position = (point / block_scale - normal / 2.0).round();
        let out_position = (point / block_scale + normal / 2.0).round();

        Self {
            point,
            normal: Direction::from(normal),
            in_coord: Coordinate::from(in_position),
            out_coord: Coordinate::from(out_position),
        }
    }
}

#[derive(Event, Clone, Copy, Debug)]
pub struct OnMouseTargetChanged {
    pub target: Option<MouseTarget>,
}

fn update_interaction_ray(
    mut mouse_interaction: ResMut<MouseInteraction>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window.single();

    mouse_interaction.ray_through_cursor = get_ray_of_active_camera(&mouse_interaction, &cameras, window);
}

fn update_mouse_target(
    rapier_context: Res<RapierContext>,
    world_settings: Res<WorldSettings>,
    mut mouse_interaction: ResMut<MouseInteraction>,
    mut on_target_changed: EventWriter<OnMouseTargetChanged>,
) {
    let target_block = calculate_mouse_target(
        &rapier_context,
        &mouse_interaction,
        world_settings.block_scale(),
    );

    if mouse_interaction.target != target_block {
        on_target_changed.send(OnMouseTargetChanged {
            target: target_block,
        });
        mouse_interaction.target = target_block;
    }
}

// Utility

fn get_ray_of_active_camera(
    mouse_interaction: &MouseInteraction,
    cameras: &Query<(&Camera, &GlobalTransform)>,
    window: &Window,
) -> Option<Ray> {
    if mouse_interaction.mouse_on_ui {
        return None;
    }

    let (camera, transform) = cameras.get(mouse_interaction.active_camera?).ok()?;

    get_cursor_as_ray(camera, transform, window)
}

fn get_cursor_as_ray(
    camera: &Camera,
    global_transform: &GlobalTransform,
    window: &Window,
) -> Option<Ray> {
    camera.viewport_to_world(global_transform, window.cursor_position()?)
}

fn calculate_mouse_target(
    rapier: &RapierContext,
    mouse_interaction: &MouseInteraction,
    block_scale: f32,
) -> Option<MouseTarget> {
    let intersection = raycast_from_ray(
        rapier,
        mouse_interaction.ray_through_cursor?,
        mouse_interaction.max_interaction_distance,
    )?;

    Some(MouseTarget::from_raycast(intersection, block_scale))
}

fn raycast_from_ray(
    rapier: &RapierContext,
    ray: Ray,
    max_distance: f32,
) -> Option<RayIntersection> {
    let intersection = rapier.cast_ray_and_get_normal(
        ray.origin,
        ray.direction,
        max_distance,
        false,
        QueryFilter::new(),
    );

    intersection.map(|(_, intersection)| intersection)
}

// Gizmos

fn draw_target_block_gizmos(
    mouse_interaction: Res<MouseInteraction>,
    world_settings: Res<WorldSettings>,
    mut gizmos: Gizmos,
) {
    if let Some(target) = mouse_interaction.target {
        let in_position = world_settings.coordinate_to_position(target.in_coord);
        let out_position = world_settings.coordinate_to_position(target.out_coord);

        gizmos.cuboid(Transform::from_translation(in_position), Color::WHITE);
        gizmos.cuboid(Transform::from_translation(out_position), Color::CYAN);
        gizmos.ray(in_position, target.normal.to_vector(), Color::BLUE);
    }
}

#[cfg(test)]
mod tests {
    use bevy_rapier3d::rapier::prelude::FeatureId;

    use crate::newtypes::direction::Direction;

    use super::*;

    #[test]
    fn can_create_mouse_target_from_raycast() {
        let intersection = RayIntersection {
            normal: Vec3::Y,
            point: Vec3::new(2.0, 1.5, 2.0),
            toi: 3.0,
            feature: FeatureId::default(),
        };

        let block_scale = 1.0;

        let target = MouseTarget::from_raycast(intersection, block_scale);

        assert_eq!(target.point, intersection.point);
        assert_eq!(target.normal, Direction::Y);
        assert_eq!(target.in_coord, Coordinate::new(2, 1, 2));
        assert_eq!(target.out_coord, Coordinate::new(2, 2, 2));
    }

    #[test]
    fn target_block_accounts_for_block_scale() {
        let intersection = RayIntersection {
            normal: Vec3::Y,
            point: Vec3::new(2.0, 1.5, 2.0),
            toi: 3.0,
            feature: FeatureId::default(),
        };

        let block_scale = 2.0;

        let target = MouseTarget::from_raycast(intersection, block_scale);

        assert_eq!(target.in_coord, Coordinate::new(1, 0, 1));
        assert_eq!(target.out_coord, Coordinate::new(1, 1, 1));
    }
}
