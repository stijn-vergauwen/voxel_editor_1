use bevy::prelude::*;

use crate::newtypes::coordinate::Coordinate;

use super::{mouse_target::MouseTarget, MouseInteraction};

pub struct MouseEventsPlugin;

impl Plugin for MouseEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OnMousePressed>()
            .add_event::<OnMouseDrag>()
            .init_resource::<InteractionSelector>()
            .add_systems(
                Update,
                (
                    handle_mouse_press,
                    send_mouse_pressed_events,
                    send_mouse_drag_events,
                )
                    .chain(),
            );
    }
}

const DISTANCE_THRESHOLD_FOR_DRAGGING: f32 = 0.3;

#[derive(Resource, Default)]
struct InteractionSelector {
    // Interaction doesn't account for which mouse button is pressed or held.
    pressed_position: Option<Vec3>,
    pressed_coord: Option<Coordinate>,
}

impl InteractionSelector {
    fn interaction_is_drag(&self, mouse_interaction: &MouseInteraction) -> bool {
        match mouse_interaction.target.map(|target| target.point) {
            Some(target_pos) => match self.pressed_position {
                Some(pressed_pos) => {
                    pressed_pos.distance(target_pos) > DISTANCE_THRESHOLD_FOR_DRAGGING
                }
                None => false,
            },
            None => false,
        }
    }

    fn interaction_is_press(&self, mouse_interaction: &MouseInteraction) -> bool {
        !self.interaction_is_drag(mouse_interaction)
    }

    fn reset(&mut self) {
        self.pressed_position = None;
        self.pressed_coord = None;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum EventPhase {
    // TODO: send mousedrag with started phase
    #[allow(unused)]
    Started,
    Ongoing,
    Ended,
}

#[derive(Event, Clone, Copy, Debug)]
pub struct OnMousePressed {
    pub button: MouseButton,
    pub on_ui: bool,
    pub target: Option<MouseTarget>,
}

#[derive(Event, Clone, Copy, Debug)]
pub struct OnMouseDrag {
    pub phase: EventPhase,
    pub button: MouseButton,
    pub start: Option<Coordinate>,
    pub end: Option<Coordinate>,
}

impl OnMouseDrag {
    pub fn drag_ended(&self) -> bool {
        self.phase == EventPhase::Ended
    }
}

fn handle_mouse_press(
    mouse_interaction: Res<MouseInteraction>,
    mut interaction_selector: ResMut<InteractionSelector>,
    input: Res<Input<MouseButton>>,
) {
    if input.get_just_pressed().len() > 0 {
        interaction_selector.pressed_position = mouse_interaction.target.map(|target| target.point);
        interaction_selector.pressed_coord = mouse_interaction.target.map(|target| target.in_coord);
    }
}

fn send_mouse_pressed_events(
    mouse_interaction: Res<MouseInteraction>,
    interaction_selector: Res<InteractionSelector>,
    input: Res<Input<MouseButton>>,
    mut on_mouse_pressed: EventWriter<OnMousePressed>,
) {
    for release in input.get_just_released() {
        if interaction_selector.interaction_is_press(&mouse_interaction) {
            on_mouse_pressed.send(OnMousePressed {
                button: *release,
                on_ui: mouse_interaction.mouse_on_ui,
                target: mouse_interaction.target,
            });
        }
    }
}

fn send_mouse_drag_events(
    mouse_interaction: Res<MouseInteraction>,
    mut interaction_selector: ResMut<InteractionSelector>,
    input: Res<Input<MouseButton>>,
    mut on_mouse_drag: EventWriter<OnMouseDrag>,
) {
    for held_button in input.get_pressed() {
        if interaction_selector.interaction_is_drag(&mouse_interaction) {
            on_mouse_drag.send(build_mouse_drag_event(
                &mouse_interaction,
                interaction_selector.pressed_coord,
                *held_button,
                EventPhase::Ongoing,
            ));
        }
    }

    for released_button in input.get_just_released() {
        if interaction_selector.interaction_is_drag(&mouse_interaction) {
            on_mouse_drag.send(build_mouse_drag_event(
                &mouse_interaction,
                interaction_selector.pressed_coord,
                *released_button,
                EventPhase::Ended,
            ));
            interaction_selector.reset();
        }
    }
}

fn build_mouse_drag_event(
    mouse_interaction: &MouseInteraction,
    start_coord: Option<Coordinate>,
    button: MouseButton,
    phase: EventPhase,
) -> OnMouseDrag {
    OnMouseDrag {
        phase,
        button,
        start: start_coord,
        end: mouse_interaction.target.map(|target| target.in_coord),
    }
}
