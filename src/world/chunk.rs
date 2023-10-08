use bevy::prelude::*;

use super::{block::Block, build_blocks_of_chunk, coordinates::ChunkIndex, CHUNK_SIZE};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Chunk {
    blocks: [[[Option<Block>; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    pub data_changed: bool,
}

impl Chunk {
    pub const EMPTY: Self = Self {
        blocks: [[[None; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
        data_changed: false,
    };

    pub fn get_block(&self, index: ChunkIndex) -> Option<Block> {
        if self.outside_bounds(index) {
            return None;
        }

        self.blocks[index.x][index.y][index.z]
    }

    pub fn set_block(&mut self, index: ChunkIndex, block: Option<Block>) {
        if self.outside_bounds(index) {
            return;
        }

        self.blocks[index.x][index.y][index.z] = block;
        self.data_changed = true;
    }

    fn outside_bounds(&self, index: ChunkIndex) -> bool {
        index.max_element() >= self.blocks.len()
    }

    pub fn flat_ground(ground_height: usize) -> Self {
        let mut chunk = Chunk::EMPTY;

        for x in 0..CHUNK_SIZE {
            for y in 0..ground_height {
                for z in 0..CHUNK_SIZE {
                    let index = ChunkIndex::new(x, y, z);
                    chunk.set_block(index, Some(Block::GRASS));
                }
            }
        }

        chunk
    }

    // TODO: remove dependency on builder (delete this method)
    pub fn generate_blocks(&mut self) -> Vec<(Block, Transform)> {
        self.data_changed = false;
        build_blocks_of_chunk(&self)
    }
}

#[cfg(test)]
mod tests {
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

        let block_id = chunk.get_block(index);

        assert_eq!(block_id, None);
    }

    #[test]
    fn can_change_block_id() {
        let mut chunk = Chunk::EMPTY;
        let index = ChunkIndex::new(0, 0, 0);

        assert_eq!(chunk.get_block(index), None);

        chunk.set_block(index, Some(Block::GRASS));

        assert_eq!(chunk.get_block(index), Some(Block::GRASS));
    }

    // TODO: coordinate to position should be a different test in a different module
    // #[test]
    // fn can_build_block_at_index() {
    //     let index = ChunkIndex::new(2, 2, 2);

    //     let block = build_block_at_index(index);

    //     let block_transform: Transform = block;
    //     let block_position = block_transform.translation;

    //     assert_eq!(block_position, Vec3::new(2.0, 2.0, 2.0))
    // }

    #[test]
    fn can_build_blocks_from_chunk() {
        let mut chunk = Chunk::EMPTY;

        chunk.set_block(ChunkIndex::new(1, 1, 1), Some(Block::GRASS));
        chunk.set_block(ChunkIndex::new(2, 6, 3), Some(Block::GRASS));

        let blocks: Vec<(Block, Transform)> = build_blocks_of_chunk(&chunk);
        let first_block_position = blocks[0].1.translation;
        let second_block_position = blocks[1].1.translation;

        assert_eq!(blocks.len(), 2);
        assert_eq!(first_block_position, Vec3::new(1.0, 1.0, 1.0));
        assert_eq!(second_block_position, Vec3::new(2.0, 6.0, 3.0));
    }

    #[test]
    fn chunk_can_be_created_as_flat_ground() {
        let ground_height = 2;

        let chunk = Chunk::flat_ground(ground_height);

        assert_eq!(
            chunk.get_block(ChunkIndex::new(0, 1, 0)),
            Some(Block::GRASS)
        );
        assert_eq!(chunk.get_block(ChunkIndex::new(0, 2, 0)), None);
    }

    #[test]
    fn chunk_tracks_if_data_changed() {
        let mut chunk = Chunk::EMPTY;

        assert_eq!(chunk.data_changed, false);

        chunk.set_block(ChunkIndex::new(1, 1, 1), Some(Block::GRASS));

        assert_eq!(chunk.data_changed, true);

        chunk.generate_blocks();

        assert_eq!(chunk.data_changed, false);
    }
}
