use bevy::prelude::*;

use crate::camera::building::{PlaceBlockRequest, RemoveBlockRequest};

use super::Chunk;

pub struct WorldInteractionPlugin;

impl Plugin for WorldInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_place_request, handle_remove_request));
    }
}

fn handle_place_request(
    mut place_events: EventReader<PlaceBlockRequest>,
    mut chunks: Query<&mut Chunk>,
) {
    for event in place_events.iter() {
        for mut chunk in chunks.iter_mut() {
            chunk.set_block_id(event.position, event.block_id);
        }
    }
}

fn handle_remove_request(
    mut remove_events: EventReader<RemoveBlockRequest>,
    mut chunks: Query<&mut Chunk>,
) {
    for event in remove_events.iter() {
        for mut chunk in chunks.iter_mut() {
            chunk.set_block_id(event.position, 0);
        }
    }
}
