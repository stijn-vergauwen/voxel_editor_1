use bevy::prelude::*;

use crate::camera::building::{OnPlaceBlockRequest, OnRemoveBlockRequest};

use crate::world::chunk::Chunk;

pub struct WorldInteractionPlugin;

impl Plugin for WorldInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_place_request, handle_remove_request));
    }
}

fn handle_place_request(
    mut place_events: EventReader<OnPlaceBlockRequest>,
    mut chunks: Query<&mut Chunk>,
) {
    for event in place_events.iter() {
        for mut chunk in chunks.iter_mut() {
            chunk.set_block(event.coord, event.block);
        }
    }
}

fn handle_remove_request(
    mut remove_events: EventReader<OnRemoveBlockRequest>,
    mut chunks: Query<&mut Chunk>,
) {
    for event in remove_events.iter() {
        for mut chunk in chunks.iter_mut() {
            chunk.set_block(event.coord, None);
        }
    }
}
