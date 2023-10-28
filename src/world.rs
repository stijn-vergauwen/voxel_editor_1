pub mod block;
mod builder;
pub mod chunk;
pub mod coordinates;
mod interaction;

use bevy::prelude::*;

use self::{
    builder::WorldBuilderPlugin, chunk::WorldChunkPlugin, coordinates::Coordinate,
    interaction::WorldInteractionPlugin,
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldSettings::new(16, 1.0))
            .add_plugins((WorldInteractionPlugin, WorldBuilderPlugin, WorldChunkPlugin));
    }
}

#[derive(Resource, Debug)]
pub struct WorldSettings {
    chunk_size: usize,
    block_scale: f32,
}

impl WorldSettings {
    pub fn new(chunk_size: usize, block_scale: f32) -> Self {
        Self {
            chunk_size,
            block_scale,
        }
    }

    pub fn block_scale(&self) -> f32 {
        self.block_scale
    }

    pub fn coordinate_to_position(&self, coord: Coordinate) -> Vec3 {
        Vec3::new(
            coord.x as f32 * self.block_scale,
            coord.y as f32 * self.block_scale,
            coord.z as f32 * self.block_scale,
        )
    }

    // pub fn position_to_coordinate(&self, position: Vec3) -> Coordinate {
    //     Coordinate::new(
    //         (position.x / self.block_scale) as usize,
    //         (position.y / self.block_scale) as usize,
    //         (position.z / self.block_scale) as usize,
    //     )
    // }
}
