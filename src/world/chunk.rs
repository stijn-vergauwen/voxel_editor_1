use bevy::prelude::*;

use super::{build_blocks_of_chunk, coordinates::ChunkIndex};

#[derive(Component)]
pub struct Chunk {
    block_ids: [[[u8; 16]; 16]; 16],
    pub data_changed: bool,
}

impl Chunk {
    pub const EMPTY: Self = Self {
        block_ids: [[[0; 16]; 16]; 16],
        data_changed: false,
    };

    pub fn get_block_id(&self, index: ChunkIndex) -> u8 {
        self.block_ids[index.x][index.y][index.z]
    }

    pub fn set_block_id(&mut self, index: ChunkIndex, id: u8) {
        self.block_ids[index.x][index.y][index.z] = id;
        self.data_changed = true;
    }

    pub fn flat_ground(ground_height: usize) -> Self {
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

    // TODO: remove dependency on builder (delete this method)
    pub fn generate_blocks(&mut self) -> Vec<Transform> {
        self.data_changed = false;
        build_blocks_of_chunk(&self)
    }
}

#[cfg(test)]
mod tests {
    use crate::world::build_block_at_index;

    use super::*;

    // TODO: move tests that don't belong in this module

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
