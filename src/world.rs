mod interaction;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use self::interaction::WorldInteractionPlugin;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInteractionPlugin)
            .add_systems(Startup, spawn_chunk)
            .add_systems(Update, redraw_changed_chunks);
    }
}

// TODO: make world size configurable.
// TODO: make block struct
// TODO: replace voxel ids array with blocks array, store it as block structs instead of ids. (don't optimize prematurely)
// TODO: block size should be adjustable
// TODO: split chunk, block, index, spawning to modules
// TODO:

#[derive(Component)]
struct Chunk {
    block_ids: [[[u8; 16]; 16]; 16],
    data_changed: bool,
}

impl Chunk {
    const EMPTY: Self = Self {
        block_ids: [[[0; 16]; 16]; 16],
        data_changed: false,
    };

    fn get_block_id(&self, index: ChunkIndex) -> u8 {
        self.block_ids[index.x][index.y][index.z]
    }

    fn set_block_id(&mut self, index: ChunkIndex, id: u8) {
        self.block_ids[index.x][index.y][index.z] = id;
        self.data_changed = true;
    }

    fn flat_ground(ground_height: usize) -> Self {
        let mut chunk = Chunk::EMPTY;

        for x in 0..16 {
            for y in 0..ground_height {
                for z in 0..16 {
                    let index = ChunkIndex::new(x, y, z);
                    chunk.set_block_id(index, 1u8);
                }
            }
        }

        chunk
    }

    fn generate_blocks(&mut self) -> Vec<Transform> {
        self.data_changed = false;
        build_blocks_of_chunk(&self)
    }
}

#[derive(Clone, Copy)]
pub struct ChunkIndex {
    x: usize,
    y: usize,
    z: usize,
}

impl ChunkIndex {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

impl From<Vec3> for ChunkIndex {
    fn from(value: Vec3) -> Self {
        ChunkIndex {
            x: value.x as usize,
            y: value.y as usize,
            z: value.z as usize,
        }
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

// Utilities

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

fn build_block_at_index(index: ChunkIndex) -> Transform {
    let position = Vec3::new(index.x as f32, index.y as f32, index.z as f32);

    Transform::from_translation(position)
}

fn build_blocks_of_chunk(chunk: &Chunk) -> Vec<Transform> {
    let mut blocks = Vec::new();

    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                let index = ChunkIndex::new(x, y, z);

                if chunk.get_block_id(index) == 1 {
                    blocks.push(build_block_at_index(index));
                }
            }
        }
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: get block
    // TODO: set block
    // TODO: change chunk size
    // TODO: get iterator over chunk
    // TODO: get iterator over solid blocks
    // TODO:

    #[test]
    fn can_get_block_id() {
        let chunk = Chunk::EMPTY;
        let index = ChunkIndex::new(0, 0, 0);

        let block_id = chunk.get_block_id(index);

        assert_eq!(block_id, 0u8);
    }

    #[test]
    fn can_change_block_id() {
        let mut chunk = Chunk::EMPTY;
        let index = ChunkIndex::new(0, 0, 0);

        assert_eq!(chunk.get_block_id(index), 0u8);

        chunk.set_block_id(index, 1u8);

        assert_eq!(chunk.get_block_id(index), 1u8);
    }

    #[test]
    fn can_build_block_at_index() {
        let index = ChunkIndex::new(2, 2, 2);

        let block = build_block_at_index(index);

        let block_transform: Transform = block;
        let block_position = block_transform.translation;

        assert_eq!(block_position, Vec3::new(2.0, 2.0, 2.0))
    }

    #[test]
    fn can_build_blocks_from_chunk() {
        let mut chunk = Chunk::EMPTY;

        chunk.set_block_id(ChunkIndex::new(1, 1, 1), 1u8);
        chunk.set_block_id(ChunkIndex::new(2, 6, 3), 1u8);

        let blocks: Vec<Transform> = build_blocks_of_chunk(&chunk);
        let first_block_position = blocks[0].translation;
        let second_block_position = blocks[1].translation;

        assert_eq!(blocks.len(), 2);
        assert_eq!(first_block_position, Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(second_block_position, Vec3::new(2.0, 6.0, 3.0));
    }

    #[test]
    fn chunk_can_be_created_as_flat_ground() {
        let ground_height = 2;

        let chunk = Chunk::flat_ground(ground_height);

        assert_eq!(chunk.get_block_id(ChunkIndex::new(0, 1, 0)), 1u8);
        assert_eq!(chunk.get_block_id(ChunkIndex::new(0, 2, 0)), 0u8);
    }

    #[test]
    fn chunk_tracks_if_data_changed() {
        let mut chunk = Chunk::EMPTY;

        assert_eq!(chunk.data_changed, false);

        chunk.set_block_id(ChunkIndex::new(1, 1, 1), 1u8);

        assert_eq!(chunk.data_changed, true);

        chunk.generate_blocks();

        assert_eq!(chunk.data_changed, false);
    }
}
