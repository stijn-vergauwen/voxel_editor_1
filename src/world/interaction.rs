use bevy::prelude::*;

use crate::camera::building::PlaceBlockRequest;

use super::{Chunk, ChunkIndex};

pub struct WorldInteractionPlugin;

impl Plugin for WorldInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_place_request);
    }
}

fn handle_place_request(
    mut place_events: EventReader<PlaceBlockRequest>,
    mut chunks: Query<&mut Chunk>,
) {
    for event in place_events.iter() {
        for mut chunk in chunks.iter_mut() {
            let index = ChunkIndex::new(
                event.position.x as usize,
                event.position.y as usize,
                event.position.z as usize,
            );

            chunk.set_block_id(index, event.block_id);
        }
    }
}
