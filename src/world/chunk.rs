use bevy::prelude::*;

use super::{block::Block, coordinates::Coordinate};

#[derive(Component, Reflect, Default, Clone, Debug)]
#[reflect(Component)]
pub struct Chunk {
    blocks: Vec<Option<Block>>,
    pub data_changed: bool,
    size: usize,
}

impl Chunk {
    pub fn empty(size: usize) -> Self {
        // TODO: with capacity & fill doesn't do what I expected, the array is empty. Fix this.
        let mut blocks = Vec::with_capacity(size * size * size);
        blocks.fill(None);

        println!("{:?}", blocks);

        Self {
            blocks,
            data_changed: false,
            size,
        }
    }

    pub fn get_block(&self, coord: Coordinate) -> Option<Block> {
        self.blocks.get(self.coordinate_to_index(coord)).cloned()?
    }

    pub fn set_block(&mut self, coord: Coordinate, new_block: Option<Block>) {
        let index = self.coordinate_to_index(coord);
        if let Some(block) = self.blocks.get_mut(index) {
            *block = new_block;
            self.data_changed = true;
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn flat_ground(ground_height: usize, color: Color, chunk_size: usize) -> Self {
        let mut chunk = Chunk::empty(chunk_size);

        for x in 0..chunk_size {
            for y in 0..ground_height {
                for z in 0..chunk_size {
                    let coord = Coordinate::new(x, y, z);
                    chunk.set_block(coord, Some(Block::new(color)));
                }
            }
        }

        chunk
    }

    fn coordinate_to_index(&self, coord: Coordinate) -> usize {
        coord.x + coord.y * self.size + coord.z * self.size * self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: get iterator over chunk
    // TODO: get iterator over solid blocks

    #[test]
    fn can_get_block_id() {
        let chunk = Chunk::empty(4);
        let coord = Coordinate::new(0, 0, 0);

        let block_id = chunk.get_block(coord);

        assert_eq!(block_id, None);
    }

    #[test]
    fn can_change_block_id() {
        let mut chunk = Chunk::empty(4);
        let coord = Coordinate::new(0, 0, 0);

        assert_eq!(chunk.get_block(coord), None);

        chunk.set_block(coord, test_block());

        assert_eq!(chunk.get_block(coord), test_block());
    }

    #[test]
    fn chunk_can_be_created_as_flat_ground() {
        let ground_height = 2;

        let chunk = Chunk::flat_ground(ground_height, Color::WHITE, 4);

        let ground_coord = Coordinate::new(0, ground_height - 1, 0);
        let empty_coord = Coordinate::new(0, ground_height, 0);

        assert_eq!(chunk.get_block(ground_coord), test_block());
        assert_eq!(chunk.get_block(empty_coord), None);
    }

    #[test]
    fn chunk_tracks_if_data_changed() {
        let mut chunk = Chunk::empty(4);

        assert_eq!(chunk.data_changed, false);

        chunk.set_block(Coordinate::new(1, 1, 1), test_block());

        assert_eq!(chunk.data_changed, true);
    }

    fn test_block() -> Option<Block> {
        Some(Block::new(Color::WHITE))
    }
}
