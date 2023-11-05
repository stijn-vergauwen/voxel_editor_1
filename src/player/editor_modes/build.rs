use bevy::prelude::*;

use crate::{
    game_systems::color_library::ColorLibrary,
    newtypes::coordinate::Coordinate,
    player::mouse_interaction::{mouse_target::MouseTarget, OnMousePressed},
    world::block::Block,
};

use super::EditorMode;

pub struct BuildModePlugin;

impl Plugin for BuildModePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnPlaceBlockRequest>()
            .add_event::<OnRemoveBlockRequest>()
            .add_systems(
                Update,
                (handle_build_input, handle_remove_input).run_if(in_state(EditorMode::Build)),
            );
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
    color_library: Res<ColorLibrary>,
    mut place_event: EventWriter<OnPlaceBlockRequest>,
) {
    for mouse_pressed in on_mouse_pressed.iter() {
        if mouse_pressed.button != BUILD_BUTTON || key_input.pressed(REMOVE_KEY) {
            continue;
        }

        if let Some(target) = mouse_pressed.target {
            send_place_block_request(&mut place_event, &color_library, target);
        }
    }
}

fn send_place_block_request(
    place_event: &mut EventWriter<OnPlaceBlockRequest>,
    color_library: &ColorLibrary,
    target: MouseTarget,
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
    key_input: Res<Input<KeyCode>>,
    mut remove_event: EventWriter<OnRemoveBlockRequest>,
) {
    for mouse_pressed in on_mouse_pressed.iter() {
        if mouse_pressed.button != BUILD_BUTTON || !key_input.pressed(REMOVE_KEY) {
            continue;
        }

        if let Some(target) = mouse_pressed.target {
            send_remove_block_request(&mut remove_event, target);
        }
    }
}

fn send_remove_block_request(
    remove_event: &mut EventWriter<OnRemoveBlockRequest>,
    target: MouseTarget,
) {
    remove_event.send(OnRemoveBlockRequest::new(Coordinate::from(target.in_coord)));
}
