use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    newtypes::{coordinate::Coordinate, direction::Direction},
    world::WorldSettings,
};

use super::{CameraInteraction, RayHit};

pub struct CameraTargetPlugin;

impl Plugin for CameraTargetPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnTargetBlockChanged>().add_systems(
            Update,
            (update_interaction_target, draw_target_block_gizmos),
        );
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TargetBlock {
    pub normal: Direction,
    pub in_coord: Coordinate,
    pub out_coord: Coordinate,
}

impl TargetBlock {
    fn from_raycast(hit: RayHit, block_scale: f32) -> Self {
        let point = hit.point;
        let normal = hit.normal;

        let in_position = (point / block_scale - normal.to_vector() / 2.0).round();
        let out_position = (point / block_scale + normal.to_vector() / 2.0).round();

        Self {
            normal: normal,
            in_coord: Coordinate::from(in_position),
            out_coord: Coordinate::from(out_position),
        }
    }
}

#[derive(Event)]
pub struct OnTargetBlockChanged {
    pub camera: Entity,
    pub new_target: Option<TargetBlock>,
}

impl OnTargetBlockChanged {
    fn new(camera: Entity, new_target: Option<TargetBlock>) -> Self {
        Self { camera, new_target }
    }
}

fn update_interaction_target(
    rapier_context: Res<RapierContext>,
    cameras: Query<(&CameraInteraction, Entity)>,
    world_settings: Res<WorldSettings>,
    mut on_target_changed: EventWriter<OnTargetBlockChanged>,
) {
    for (interaction, entity) in cameras.iter() {
        let block_scale = world_settings.block_scale();
        let target_block = cast_ray_to_target_block(&rapier_context, &interaction, block_scale);

        if interaction.target != target_block {
            on_target_changed.send(OnTargetBlockChanged::new(entity, target_block));
        }
    }
}

fn draw_target_block_gizmos(
    cameras: Query<&CameraInteraction>,
    mut gizmos: Gizmos,
    world_settings: Res<WorldSettings>,
) {
    for camera in cameras.iter() {
        if let Some(target) = &camera.target {
            let in_position = world_settings.coordinate_to_position(target.in_coord);
            let out_position = world_settings.coordinate_to_position(target.out_coord);

            gizmos.cuboid(Transform::from_translation(in_position), Color::WHITE);
            gizmos.cuboid(Transform::from_translation(out_position), Color::CYAN);
            gizmos.ray(in_position, target.normal.to_vector(), Color::BLUE);
        }
    }
}

// Utility

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
    use crate::newtypes::direction::Direction;

    use super::*;

    #[test]
    fn can_create_target_block_from_raycast() {
        let ray_hit = RayHit::new(Vec3::new(1.0, 0.0, 1.8), Direction::X);
        let block_scale = 1.0;

        let target_block = TargetBlock::from_raycast(ray_hit, block_scale);

        assert_eq!(target_block.normal, Direction::X);
    }

    #[test]
    fn target_block_calculates_in_coord() {
        let block_scale = 1.0;
        let target_block =
            TargetBlock::from_raycast(RayHit::new(Vec3::new(1.5, 0.0, 1.8), Direction::X), block_scale);

        assert_eq!(target_block.in_coord, Coordinate::new(1, 0, 2));

        let target_block =
            TargetBlock::from_raycast(RayHit::new(Vec3::new(7.8, 3.4, 7.2), Direction::Y), block_scale);

        assert_eq!(target_block.in_coord, Coordinate::new(8, 3, 7));
    }

    #[test]
    fn target_block_calculates_out_coord() {
        let block_scale = 1.0;
        let target_block =
            TargetBlock::from_raycast(RayHit::new(Vec3::new(3.5, 0.0, 2.8), Direction::X), block_scale);

        assert_eq!(target_block.out_coord, Coordinate::new(4, 0, 3));

        let target_block =
            TargetBlock::from_raycast(RayHit::new(Vec3::new(5.8, 2.4, 3.2), Direction::Y), block_scale);

        assert_eq!(target_block.out_coord, Coordinate::new(6, 3, 3));
    }

    #[test]
    fn target_block_accounts_for_block_scale() {
        let ray_hit = RayHit::new(Vec3::new(3.0, 0.0, 0.0), Direction::X);
        let block_scale = 2.0;

        let target_block = TargetBlock::from_raycast(ray_hit, block_scale);

        assert_eq!(target_block.in_coord, Coordinate::new(1, 0, 0));
        assert_eq!(target_block.out_coord, Coordinate::new(2, 0, 0));

        let ray_hit = RayHit::new(Vec3::new(4.0, 0.0, 2.0), Direction::X);
        let block_scale = 3.5;

        let target_block = TargetBlock::from_raycast(ray_hit, block_scale);

        assert_eq!(target_block.in_coord, Coordinate::new(1, 0, 1));
        assert_eq!(target_block.out_coord, Coordinate::new(2, 0, 1));
    }
}
