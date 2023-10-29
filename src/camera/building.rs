use bevy::prelude::*;
use flying_camera::FlyingCamera;

use crate::{color_library::ColorLibrary, newtypes::coordinate::Coordinate, world::block::Block};

use super::{CameraInteraction, TargetBlock};

pub struct CameraBuildingPlugin;

impl Plugin for CameraBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnPlaceBlockRequest>()
            .add_event::<OnRemoveBlockRequest>()
            .add_systems(Update, (send_build_event, send_remove_event));
    }
}

const BUILD_BUTTON: MouseButton = MouseButton::Left;
const REMOVE_KEY: KeyCode = KeyCode::ShiftLeft;

#[derive(Event)]
pub struct OnPlaceBlockRequest {
    pub block: Option<Block>,
    pub coord: Coordinate,
}

impl OnPlaceBlockRequest {
    fn new(block: Option<Block>, coord: Coordinate) -> Self {
        Self { block, coord }
    }
}

#[derive(Event)]
pub struct OnRemoveBlockRequest {
    pub coord: Coordinate,
}

impl OnRemoveBlockRequest {
    fn new(coord: Coordinate) -> Self {
        Self { coord }
    }
}

fn send_build_event(
    mut place_event: EventWriter<OnPlaceBlockRequest>,
    cameras: Query<(&CameraInteraction, &FlyingCamera)>,
    mouse_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
    color_library: Res<ColorLibrary>,
) {
    if let Some(target) = get_valid_interaction_target(&cameras) {
        if mouse_input.just_pressed(BUILD_BUTTON) && !key_input.pressed(REMOVE_KEY) {
            place_event.send(OnPlaceBlockRequest::new(
                color_library
                    .selected_color()
                    .map(|color| Block::new(color)),
                target.out_coord,
            ));
        }
    }
}

fn send_remove_event(
    mut remove_event: EventWriter<OnRemoveBlockRequest>,
    cameras: Query<(&CameraInteraction, &FlyingCamera)>,
    mouse_input: Res<Input<MouseButton>>,
    key_input: Res<Input<KeyCode>>,
) {
    if let Some(target) = get_valid_interaction_target(&cameras) {
        if mouse_input.just_pressed(BUILD_BUTTON) && key_input.pressed(REMOVE_KEY) {
            remove_event.send(OnRemoveBlockRequest::new(Coordinate::from(target.in_coord)));
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
