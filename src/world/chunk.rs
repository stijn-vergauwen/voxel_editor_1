use bevy::prelude::*;

use super::{block::Block, build_blocks_of_chunk, coordinates::Coordinate, CHUNK_SIZE};

#[derive(Component, Reflect, Default, Clone, Copy, Debug)]
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

    pub fn get_block(&self, coord: Coordinate) -> Option<Block> {
        if self.outside_bounds(coord) {
            return None;
        }

        self.blocks[coord.x][coord.y][coord.z]
    }

    pub fn set_block(&mut self, coord: Coordinate, block: Option<Block>) {
        if self.outside_bounds(coord) {
            return;
        }

        self.blocks[coord.x][coord.y][coord.z] = block;
        self.data_changed = true;
    }

    fn outside_bounds(&self, coord: Coordinate) -> bool {
        coord.max_element() >= self.blocks.len()
    }

    pub fn flat_ground(ground_height: usize, color: Color) -> Self {
        let mut chunk = Chunk::EMPTY;

        for x in 0..CHUNK_SIZE {
            for y in 0..ground_height {
                for z in 0..CHUNK_SIZE {
                    let coord = Coordinate::new(x, y, z);
                    chunk.set_block(coord, Some(Block::new(color)));
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

    // TODO: change chunk size
    // TODO: get iterator over chunk
    // TODO: get iterator over solid blocks
    // TODO:

    #[test]
    fn can_get_block_id() {
        let chunk = Chunk::EMPTY;
        let coord = Coordinate::new(0, 0, 0);

        let block_id = chunk.get_block(coord);

        assert_eq!(block_id, None);
    }

    #[test]
    fn can_change_block_id() {
        let mut chunk = Chunk::EMPTY;
        let coord = Coordinate::new(0, 0, 0);

        assert_eq!(chunk.get_block(coord), None);

        chunk.set_block(coord, test_block());

        assert_eq!(chunk.get_block(coord), test_block());
    }

    // TODO: coordinate to position should be a different test in a different module
    // #[test]
    // fn can_build_block_at_coord() {
    //     let coord = ChunkIndex::new(2, 2, 2);

    //     let block = build_block_at_index(index);

    //     let block_transform: Transform = block;
    //     let block_position = block_transform.translation;

    //     assert_eq!(block_position, Vec3::new(2.0, 2.0, 2.0))
    // }

    #[test]
    fn can_build_blocks_from_chunk() {
        let mut chunk = Chunk::EMPTY;

        chunk.set_block(Coordinate::new(1, 1, 1), test_block());
        chunk.set_block(Coordinate::new(2, 6, 3), test_block());

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

        let chunk = Chunk::flat_ground(ground_height, Color::WHITE);

        assert_eq!(chunk.get_block(Coordinate::new(0, 1, 0)), test_block());
        assert_eq!(chunk.get_block(Coordinate::new(0, 2, 0)), None);
    }

    #[test]
    fn chunk_tracks_if_data_changed() {
        let mut chunk = Chunk::EMPTY;

        assert_eq!(chunk.data_changed, false);

        chunk.set_block(Coordinate::new(1, 1, 1), test_block());

        assert_eq!(chunk.data_changed, true);

        chunk.generate_blocks();

        assert_eq!(chunk.data_changed, false);
    }

    fn test_block() -> Option<Block> {
        Some(Block::new(Color::WHITE))
    }
}
