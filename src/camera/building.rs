use bevy::prelude::*;
use flying_camera::FlyingCamera;

use crate::world::{coordinates::ChunkIndex, block::Block};

use super::CameraInteraction;

pub struct CameraBuildingPlugin;

impl Plugin for CameraBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaceBlockRequest>()
            .add_event::<RemoveBlockRequest>()
            .add_systems(Update, send_build_event);
    }
}

const BUILD_BUTTON: MouseButton = MouseButton::Left;
const REMOVE_KEY: KeyCode = KeyCode::ShiftLeft;

#[derive(Event)]
pub struct PlaceBlockRequest {
    pub block: Option<Block>,
    pub position: ChunkIndex,
}

impl PlaceBlockRequest {
    fn new(block: Option<Block>, position: ChunkIndex) -> Self {
        Self { block, position }
    }
}

#[derive(Event)]
pub struct RemoveBlockRequest {
    pub position: ChunkIndex,
}

impl RemoveBlockRequest {
    fn new(position: ChunkIndex) -> Self {
        Self { position }
    }
}

fn send_build_event(
    mut place_event: EventWriter<PlaceBlockRequest>,
    mut remove_event: EventWriter<RemoveBlockRequest>,
    cameras: Query<(&CameraInteraction, &FlyingCamera)>,
    mouse_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
) {
    if let Ok((camera_interaction, camera)) = cameras.get_single() {
        if let Some(target) = &camera_interaction.target {
            if !camera.enabled && mouse_input.just_pressed(BUILD_BUTTON) {
                if key_input.pressed(REMOVE_KEY) {
                    remove_event.send(RemoveBlockRequest::new(ChunkIndex::from(
                        target.in_position,
                    )));
                } else {
                    place_event.send(PlaceBlockRequest::new(
                        Some(Block::GRASS),
                        ChunkIndex::from(target.out_position),
                    ));
                }
            }
        }
    }
}
