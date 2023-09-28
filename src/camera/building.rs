use bevy::prelude::*;
use flying_camera::FlyingCamera;

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
    block_id: u8,
    position: Vec3,
}

impl PlaceBlockRequest {
    fn new(block_id: u8, position: Vec3) -> Self {
        Self { block_id, position }
    }
}

#[derive(Event)]
pub struct RemoveBlockRequest {
    position: Vec3,
}

impl RemoveBlockRequest {
    fn new(position: Vec3) -> Self {
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
                    remove_event.send(RemoveBlockRequest::new(target.in_position));
                } else {
                    place_event.send(PlaceBlockRequest::new(1, target.out_position));
                }
            }
        }
    }
}
