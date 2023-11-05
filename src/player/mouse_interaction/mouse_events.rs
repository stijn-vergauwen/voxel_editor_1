use bevy::prelude::*;

use super::{mouse_target::MouseTarget, MouseInteraction};

pub struct MouseEventsPlugin;

impl Plugin for MouseEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnMousePressed>()
            .add_event::<OnMouseReleased>()
            .add_systems(Update, (send_mouse_pressed_events, send_mouse_released_events));
    }
}

#[derive(Event, Clone, Copy, Debug)]
pub struct OnMousePressed {
    pub button: MouseButton,
    pub on_ui: bool,
    pub target: Option<MouseTarget>,
}

#[derive(Event, Clone, Copy, Debug)]
pub struct OnMouseReleased {
    pub button: MouseButton,
    pub on_ui: bool,
    pub target: Option<MouseTarget>,
}

fn send_mouse_pressed_events(
    mouse_interaction: Res<MouseInteraction>,
    input: Res<Input<MouseButton>>,
    mut on_mouse_pressed: EventWriter<OnMousePressed>,
) {
    for press in input.get_just_pressed() {
        on_mouse_pressed.send(OnMousePressed {
            button: *press,
            on_ui: mouse_interaction.mouse_on_ui,
            target: mouse_interaction.target,
        });
    }
}

fn send_mouse_released_events(
    mouse_interaction: Res<MouseInteraction>,
    input: Res<Input<MouseButton>>,
    mut on_mouse_released: EventWriter<OnMouseReleased>,
) {
    for release in input.get_just_released() {
        on_mouse_released.send(OnMouseReleased {
            button: *release,
            on_ui: mouse_interaction.mouse_on_ui,
            target: mouse_interaction.target,
        });
    }
}
