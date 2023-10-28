use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{
    block::Block,
    chunk::{Chunk, OnRedrawChunkRequest},
    WorldSettings,
};

pub struct WorldBuilderPlugin;

impl Plugin for WorldBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_chunk)
            .add_systems(Update, redraw_requested_chunks);
    }
}

fn spawn_chunk(mut commands: Commands, world_settings: Res<WorldSettings>) {
    let ground_height = 2;

    commands.spawn((
        Name::new("Chunk"),
        SpatialBundle::default(),
        Chunk::flat_ground(ground_height, Color::LIME_GREEN, world_settings.chunk_size),
    ));
}

fn redraw_requested_chunks(
    mut commands: Commands,
    chunks: Query<&Chunk>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world_settings: Res<WorldSettings>,
    mut chunk_redraw_requests: EventReader<OnRedrawChunkRequest>,
) {
    for request in chunk_redraw_requests.iter() {
        let chunk_entity = request.chunk;

        if let Ok(chunk) = chunks.get(chunk_entity) {
            // Remove blocks
            commands.entity(chunk_entity).despawn_descendants();

            // draw blocks
            let blocks = calculate_blocks_spawn_positions(&chunk, &world_settings);

            draw_chunk(
                &mut commands,
                &mut meshes,
                &mut materials,
                blocks,
                chunk_entity,
            );
        }
    }
}

fn draw_chunk(
    mut commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    blocks: Vec<(Block, Vec3)>,
    chunk_entity: Entity,
) {
    let mesh_handle = meshes.add(shape::Cube::new(0.9).into());

    let block_entities = spawn_blocks(&mut commands, blocks, mesh_handle, materials);

    commands.entity(chunk_entity).push_children(&block_entities);
}

fn spawn_blocks(
    commands: &mut Commands,
    blocks: Vec<(Block, Vec3)>,
    mesh_handle: Handle<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Vec<Entity> {
    let mut spawned_entities = Vec::new();

    for (block, position) in blocks.iter() {
        let material_handle = materials.add(StandardMaterial {
            base_color: block.color,
            ..default()
        });

        let id = commands
            .spawn(build_block(&mesh_handle, &material_handle, *position))
            .id();

        spawned_entities.push(id);
    }

    spawned_entities
}

fn calculate_blocks_spawn_positions(
    chunk: &Chunk,
    world_settings: &WorldSettings,
) -> Vec<(Block, Vec3)> {
    chunk
        .get_assigned_blocks_with_coords()
        .into_iter()
        .map(|(block, coord)| (block, world_settings.coordinate_to_position(coord)))
        .collect()
}

fn build_block(
    mesh_handle: &Handle<Mesh>,
    material_handle: &Handle<StandardMaterial>,
    position: Vec3,
) -> (PbrBundle, Collider) {
    (
        PbrBundle {
            mesh: mesh_handle.clone(),
            material: material_handle.clone(),
            transform: Transform::from_translation(position),
            ..default()
        },
        Collider::cuboid(0.5, 0.5, 0.5),
    )
}
