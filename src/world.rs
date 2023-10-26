pub mod block;
mod builder;
pub mod chunk;
pub mod coordinates;
mod interaction;

use bevy::prelude::*;

use self::{
    block::Block, builder::WorldBuilderPlugin, chunk::Chunk, coordinates::Coordinate,
    interaction::WorldInteractionPlugin,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldSettings::new(16, 1.0))
            .register_type::<Chunk>()
            .register_type::<Block>()
            .add_plugins((WorldInteractionPlugin, WorldBuilderPlugin));
    }
}

#[derive(Resource, Debug)]
pub struct WorldSettings {
    pub chunk_size: usize,
    pub block_scale: f32,
}

impl WorldSettings {
    pub fn new(chunk_size: usize, block_scale: f32) -> Self {
        Self {
            chunk_size,
            block_scale,
        }
    }
}

// TODO: block size should be adjustable
// TODO: coordinate to position helper functions
// TODO: event for when a chunk needs to update

// Utilities

// TODO: These build functions don't make sense, replace with coord to position utils
fn build_block_at_coordinate(coord: Coordinate, block: Block) -> (Block, Transform) {
    let position = Vec3::new(coord.x as f32, coord.y as f32, coord.z as f32);

    (block, Transform::from_translation(position))
}

fn build_blocks_of_chunk(chunk: &Chunk) -> Vec<(Block, Transform)> {
    let mut blocks = Vec::new();

    for x in 0..chunk.size() {
        for y in 0..chunk.size() {
            for z in 0..chunk.size() {
                let coord = Coordinate::new(x, y, z);

                if let Some(block) = chunk.get_block(coord) {
                    blocks.push(build_block_at_coordinate(coord, block));
                }
            }
        }
    }

    blocks
}
