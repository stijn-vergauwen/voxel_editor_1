use bevy::prelude::*;
use flying_camera::FlyingCamera;

use crate::{
    color_library::ColorLibrary,
    world::{block::Block, coordinates::ChunkIndex},
};

use super::{CameraInteraction, TargetBlock};

pub struct CameraBuildingPlugin;

impl Plugin for CameraBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaceBlockRequest>()
            .add_event::<RemoveBlockRequest>()
            .add_systems(Update, (send_build_event, send_remove_event));
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
    cameras: Query<(&CameraInteraction, &FlyingCamera)>,
    mouse_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    color_library: Res<ColorLibrary>,
) {
    if let Some(target) = get_valid_interaction_target(&cameras) {
        if mouse_input.just_pressed(BUILD_BUTTON) && !key_input.pressed(REMOVE_KEY) {
            place_event.send(PlaceBlockRequest::new(
                color_library
                    .selected_color()
                    .map(|color| Block::new(color)),
                ChunkIndex::from(target.out_position),
            ));
        }
    }
}

fn send_remove_event(
    mut remove_event: EventWriter<RemoveBlockRequest>,
    cameras: Query<(&CameraInteraction, &FlyingCamera)>,
    mouse_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
) {
    if let Some(target) = get_valid_interaction_target(&cameras) {
        if mouse_input.just_pressed(BUILD_BUTTON) && key_input.pressed(REMOVE_KEY) {
            remove_event.send(RemoveBlockRequest::new(ChunkIndex::from(
                target.in_position,
            )));
        }
    }
}

fn get_valid_interaction_target(
    camera_query: &Query<(&CameraInteraction, &FlyingCamera)>,
) -> Option<TargetBlock> {
    if let Ok((camera_interaction, camera)) = camera_query.get_single() {
        if !camera.enabled {
            return camera_interaction.target;
        }
    }
    None
}
