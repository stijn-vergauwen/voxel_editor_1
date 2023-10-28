use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{block::Block, chunk::Chunk, WorldSettings};

pub struct WorldBuilderPlugin;

impl Plugin for WorldBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_chunk)
            .add_systems(Update, redraw_changed_chunks);
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

fn redraw_changed_chunks(
    mut commands: Commands,
    mut chunks: Query<(&mut Chunk, Entity), Changed<Chunk>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world_settings: Res<WorldSettings>,
) {
    for (mut chunk, chunk_entity) in chunks.iter_mut().filter(|(chunk, _)| chunk.data_changed) {
        // TODO: split calculations to function
        // Remove blocks
        commands.entity(chunk_entity).despawn_descendants();

        let blocks = calculate_blocks_spawn_data(&chunk, &world_settings);
        chunk.data_changed = false;

        // Spawn blocks
        let mesh_handle = meshes.add(shape::Cube::new(0.9).into());

        let block_entities = spawn_chunk_blocks(&mut commands, blocks, mesh_handle, &mut materials);

        commands.entity(chunk_entity).push_children(&block_entities);
    }
}

fn spawn_chunk_blocks(
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
            .spawn((
                PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: material_handle.clone(),
                    transform: Transform::from_translation(*position),
                    ..default()
                },
                Collider::cuboid(0.5, 0.5, 0.5),
            ))
            .id();

        spawned_entities.push(id);
    }

    spawned_entities
}

fn calculate_blocks_spawn_data(
    chunk: &Chunk,
    world_settings: &WorldSettings,
) -> Vec<(Block, Vec3)> {
    chunk
        .get_assigned_blocks_with_coords()
        .into_iter()
        .map(|(block, coord)| (block, world_settings.coordinate_to_position(coord)))
        .collect()
}
