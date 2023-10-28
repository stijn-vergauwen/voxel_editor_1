use bevy::prelude::*;

use super::{block::Block, coordinates::Coordinate};

pub struct WorldChunkPlugin;

impl Plugin for WorldChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnRedrawChunkRequest>()
            .add_systems(Update, check_chunks_that_changed);
    }
}

#[derive(Component, Default, Clone, Debug)]
pub struct Chunk {
    blocks: Vec<Option<Block>>,
    data_changed: bool,
    size: usize,
}

impl Chunk {
    pub fn empty(size: usize) -> Self {
        let block_count = size * size * size;
        let mut blocks = Vec::new();
        blocks.resize(block_count, None);

        Self {
            blocks,
            data_changed: false,
            size,
        }
    }

    #[allow(unused)]
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

    pub fn get_assigned_blocks_with_coords(&self) -> Vec<(Block, Coordinate)> {
        self.blocks
            .iter()
            .enumerate()
            .filter_map(|(index, block)| {
                block.map(|block| (block, self.index_to_coordinate(index)))
            })
            .collect()
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

    pub fn coordinate_to_index(&self, coord: Coordinate) -> usize {
        coord.x + coord.y * self.size + coord.z * self.size * self.size
    }

    pub fn index_to_coordinate(&self, index: usize) -> Coordinate {
        let z = index / (self.size * self.size);
        let z_remainder = index % (self.size * self.size);
        let y = z_remainder / self.size;
        let y_remainder = z_remainder % self.size;
        let x = y_remainder;

        Coordinate::new(x, y, z)
    }
}

#[derive(Event)]
pub struct OnRedrawChunkRequest {
    pub chunk: Entity,
}

impl OnRedrawChunkRequest {
    fn new(chunk: Entity) -> Self {
        Self { chunk }
    }
}

fn check_chunks_that_changed(
    mut chunks: Query<(&mut Chunk, Entity)>,
    mut on_redraw_request: EventWriter<OnRedrawChunkRequest>,
) {
    for (mut chunk, entity) in chunks.iter_mut().filter(|(chunk, _)| chunk.data_changed) {
        on_redraw_request.send(OnRedrawChunkRequest::new(entity));
        chunk.data_changed = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn created_chunk_has_correct_block_count() {
        let chunk = Chunk::empty(3);

        assert_eq!(chunk.blocks.len(), 27);

        let chunk = Chunk::empty(5);

        assert_eq!(chunk.blocks.len(), 125);

        let chunk = Chunk::empty(10);

        assert_eq!(chunk.blocks.len(), 1000);
    }

    #[test]
    fn can_get_block() {
        let chunk = Chunk::empty(4);
        let coord = Coordinate::new(0, 0, 0);

        let block = chunk.get_block(coord);

        assert_eq!(block, None);
    }

    #[test]
    fn can_change_block() {
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

    #[test]
    fn can_calculate_index_from_coordinate() {
        let size = 4;
        let chunk = Chunk::empty(size);

        assert_eq!(chunk.coordinate_to_index(Coordinate::new(3, 0, 0)), 3);
        assert_eq!(chunk.coordinate_to_index(Coordinate::new(2, 2, 2)), 42);

        let size = 10;
        let chunk = Chunk::empty(size);

        assert_eq!(chunk.coordinate_to_index(Coordinate::new(3, 0, 0)), 3);
        assert_eq!(chunk.coordinate_to_index(Coordinate::new(2, 2, 2)), 222);
    }

    #[test]
    fn can_calculate_coordinate_from_index() {
        let size = 4;
        let chunk = Chunk::empty(size);

        assert_eq!(chunk.index_to_coordinate(2), Coordinate::new(2, 0, 0));
        assert_eq!(chunk.index_to_coordinate(58), Coordinate::new(2, 2, 3));

        let size = 10;
        let chunk = Chunk::empty(size);

        assert_eq!(chunk.index_to_coordinate(3), Coordinate::new(3, 0, 0));
        assert_eq!(chunk.index_to_coordinate(232), Coordinate::new(2, 3, 2));
    }

    fn test_block() -> Option<Block> {
        Some(Block::new(Color::WHITE))
    }
}
