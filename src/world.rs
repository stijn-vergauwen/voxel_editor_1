pub mod block;
mod builder;
mod chunk;
pub mod coordinates;
mod interaction;

use bevy::prelude::*;

use self::{
    builder::WorldBuilderPlugin, chunk::Chunk, coordinates::ChunkIndex,
    interaction::WorldInteractionPlugin, block::Block,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WorldInteractionPlugin, WorldBuilderPlugin));
    }
}

const CHUNK_SIZE: usize = 16;

// TODO: block size should be adjustable
// TODO: event for when a chunk needs to update
// TODO:

// Utilities

// TODO: These transform returns don't make sense, replace with index to position utils
fn build_block_at_index(index: ChunkIndex, block: Block) -> (Block, Transform)  {
    let position = Vec3::new(index.x as f32, index.y as f32, index.z as f32);

    (block, Transform::from_translation(position))
}

fn build_blocks_of_chunk(chunk: &Chunk) -> Vec<(Block, Transform)> {
    let mut blocks = Vec::new();

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let index = ChunkIndex::new(x, y, z);

                if let Some(block) = chunk.get_block(index) {
                    blocks.push(build_block_at_index(index, block));
                }
            }
        }
    }

    blocks
}
