use bevy::prelude::*;

use crate::{
    game_systems::color_library::ColorLibrary,
    newtypes::coordinate::Coordinate,
    player::mouse_interaction::{
        mouse_events::{OnMouseDrag, OnMousePressed},
        mouse_target::MouseTarget,
    },
    world::block::Block,
};

use super::{select::get_coordinates_between, EditorMode};

pub struct BuildModePlugin;

impl Plugin for BuildModePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnPlaceBlockRequest>()
            .add_event::<OnRemoveBlockRequest>()
            .add_systems(
                Update,
                (handle_mouse_press, handle_mouse_drag, handle_remove_input)
                    .run_if(in_state(EditorMode::Build)),
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

fn handle_mouse_press(
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
            send_place_block_request(&mut place_event, &color_library, target.out_coord);
        }
    }
}

fn handle_mouse_drag(
    mut on_mouse_drag: EventReader<OnMouseDrag>,
    key_input: Res<Input<KeyCode>>,
    color_library: Res<ColorLibrary>,
    mut place_event: EventWriter<OnPlaceBlockRequest>,
) {
    for mouse_drag in on_mouse_drag.iter() {
        if mouse_drag.button != BUILD_BUTTON
            || !mouse_drag.drag_ended()
            || key_input.pressed(REMOVE_KEY)
        {
            continue;
        }

        if let (Some(start), Some(end)) = (
            mouse_drag.start.map(|target| target.out_coord),
            mouse_drag.end.map(|target| target.out_coord),
        ) {
            for coord in get_coordinates_between(start, end).into_iter() {
                send_place_block_request(&mut place_event, &color_library, coord);
            }
        }
    }
}

fn send_place_block_request(
    place_event: &mut EventWriter<OnPlaceBlockRequest>,
    color_library: &ColorLibrary,
    coord: Coordinate,
) {
    place_event.send(OnPlaceBlockRequest::new(
        color_library
            .selected_color()
            .map(|color| Block::new(color)),
        coord,
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
