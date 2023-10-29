use bevy::prelude::*;
use flying_camera::FlyingCamera;

use crate::{
    color_library::ColorLibrary, mouse_interaction::OnMousePressed,
    newtypes::coordinate::Coordinate, world::block::Block,
};

use super::{CameraInteraction, TargetBlock};

pub struct CameraBuildingPlugin;

impl Plugin for CameraBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnPlaceBlockRequest>()
            .add_event::<OnRemoveBlockRequest>()
            .add_systems(Update, (handle_build_input, handle_remove_input));
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

fn handle_build_input(
    mut on_mouse_pressed: EventReader<OnMousePressed>,
    key_input: Res<Input<KeyCode>>,
    cameras: Query<(&CameraInteraction, &FlyingCamera)>,
    color_library: Res<ColorLibrary>,
    mut place_event: EventWriter<OnPlaceBlockRequest>,
) {
    for mouse_pressed in on_mouse_pressed.iter() {
        if let Some(target) = get_valid_interaction_target(&cameras) {
            let pressing_correct_keys =
                mouse_pressed.button == BUILD_BUTTON && !key_input.pressed(REMOVE_KEY);

            if !mouse_pressed.on_ui && pressing_correct_keys {
                send_place_block_request(&mut place_event, &color_library, target);
            }
        }
    }
}

fn send_place_block_request(
    place_event: &mut EventWriter<OnPlaceBlockRequest>,
    color_library: &ColorLibrary,
    target: TargetBlock,
) {
    place_event.send(OnPlaceBlockRequest::new(
        color_library
            .selected_color()
            .map(|color| Block::new(color)),
        target.out_coord,
    ));
}

fn handle_remove_input(
    mut on_mouse_pressed: EventReader<OnMousePressed>,
    cameras: Query<(&CameraInteraction, &FlyingCamera)>,
    key_input: Res<Input<KeyCode>>,
    mut remove_event: EventWriter<OnRemoveBlockRequest>,
) {
    for mouse_pressed in on_mouse_pressed.iter() {
        if let Some(target) = get_valid_interaction_target(&cameras) {
            let pressing_correct_keys =
                mouse_pressed.button == BUILD_BUTTON && key_input.pressed(REMOVE_KEY);

            if !mouse_pressed.on_ui && pressing_correct_keys {
                send_remove_block_request(&mut remove_event, target);
            }
        }
    }
}

fn send_remove_block_request(
    remove_event: &mut EventWriter<OnRemoveBlockRequest>,
    target: TargetBlock,
) {
    remove_event.send(OnRemoveBlockRequest::new(Coordinate::from(target.in_coord)));
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
