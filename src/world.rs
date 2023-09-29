pub mod block;
mod builder;
mod chunk;
pub mod coordinates;
mod interaction;

use bevy::prelude::*;

use self::{
    builder::WorldBuilderPlugin, chunk::Chunk, coordinates::ChunkIndex,
    interaction::WorldInteractionPlugin,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WorldInteractionPlugin, WorldBuilderPlugin));
    }
}

// TODO: make world size configurable.
// TODO: replace voxel ids array with blocks array, store it as block structs instead of ids. (don't optimize prematurely)
// TODO: block size should be adjustable
// TODO: event for when a chunk needs to update
// TODO: block id newtype
// TODO:

// Utilities

// TODO: These transform returns don't make sense, replace with index to position utils
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

                if chunk.get_block(index).is_some() {
                    blocks.push(build_block_at_index(index));
                }
            }
        }
    }

    blocks
}
