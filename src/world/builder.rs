use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::chunk::Chunk;

pub struct WorldBuilderPlugin;

impl Plugin for WorldBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_chunk)
            .add_systems(Update, redraw_changed_chunks);
    }
}

fn spawn_chunk(mut commands: Commands) {
    let ground_height = 2;

    commands.spawn((SpatialBundle::default(), Chunk::flat_ground(ground_height)));
}

fn redraw_changed_chunks(
    mut commands: Commands,
    mut chunks: Query<(&mut Chunk, Entity), Changed<Chunk>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut chunk, chunk_entity) in chunks.iter_mut().filter(|(chunk, _)| chunk.data_changed) {
        println!("Redraw chunk");

        // Remove blocks
        commands.entity(chunk_entity).despawn_descendants();

        // Generate blocks
        let blocks = chunk.generate_blocks();

        // Spawn blocks
        let mesh_handle = meshes.add(shape::Cube::new(0.9).into());
        let material_handle = materials.add(StandardMaterial {
            base_color: Color::LIME_GREEN,
            ..default()
        });

        let block_entities =
            spawn_chunk_blocks(&mut commands, blocks, mesh_handle, material_handle);

        commands.entity(chunk_entity).push_children(&block_entities);
    }
}

fn spawn_chunk_blocks(
    commands: &mut Commands,
    blocks: Vec<Transform>,
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<StandardMaterial>,
) -> Vec<Entity> {
    let mut spawned_entities = Vec::new();

    for block in blocks.iter() {
        let id = commands
            .spawn((
                PbrBundle {
                    mesh: mesh_handle.clone(),
                    material: material_handle.clone(),
                    transform: *block,
                    ..default()
                },
                Collider::cuboid(0.5, 0.5, 0.5),
            ))
            .id();

        spawned_entities.push(id);
    }

    spawned_entities
}