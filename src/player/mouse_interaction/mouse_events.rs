use bevy::prelude::*;

use super::{mouse_target::MouseTarget, MouseInteraction};

/*
    Tips for next iteration:

    - Build these events up with multiple layers, each containing more abstract data
        - low layers have fewer events and hold simple data in struct
        - higher layers have many events with less, more specific data
        - game systems respond to higher layer events, higher layer events respond to lower layer

    After reading this ^ section, this basically is the 'Command' pattern:
    - Split events into 3 layers:
        - describing each possible action as an event
            - This describes what in-game thing happens, not how or where it was called
            - These events can probably be spread throughout the project, at the place it's used, not sent.
        - mapping keypresses to actions
            - A large list of objects that describe requirements to activate, and which action happens
            - Rebinding keys is done by changing this list
            - Inputs & actions can be many-to-many
        - resources holding real-time world info 
            - e.g.: mouse position, game-state, current screen or menu
            - maybe access this info through &World? since different actions can have very different requirements
    - How this will work is:
        - 1. a key or button is pressed, this is wrapped in a struct describing the button & press phase
        - 2. every key - action mapping is checked, one's that match are stored for next step
        - 3. action requirements are checked, things like: 'is cursor on ui?', 'does this entity / player exist?', 'are we in-game or in a menu?'.
        - 4. if all requirements are met, the action event is sent with all it's relevant data.
    - Some things I don't know yet:
        - If all actions begin in the same module, how is the EventWriter called? does each event need it's own system to send events, or does 1 system need access to every event writer?
            - maybe send each action through a ActionEvent trait, and maybe if a systems accesses World it can loop though available events? I think this solves the code repetition
            - key-action mapping probably also needs &World access, since each object needs to check different world info
        - Is there too much input delay if there are multiple steps? Or can this all be done in one cycle?
*/

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
    start_target: Option<MouseTarget>,
}

impl InteractionSelector {
    fn interaction_is_drag(&self, mouse_interaction: &MouseInteraction) -> bool {
        match mouse_interaction.target {
            Some(end_target) => match self.start_target {
                Some(start_target) => {
                    start_target.point.distance(end_target.point) > DISTANCE_THRESHOLD_FOR_DRAGGING
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
        self.start_target = None;
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
    pub start: Option<MouseTarget>,
    pub end: Option<MouseTarget>,
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
        interaction_selector.start_target = mouse_interaction.target;
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
            on_mouse_drag.send({
                OnMouseDrag {
                    phase: EventPhase::Ongoing,
                    button: *held_button,
                    start: interaction_selector.start_target,
                    end: mouse_interaction.target,
                }
            });
        }
    }

    for released_button in input.get_just_released() {
        if interaction_selector.interaction_is_drag(&mouse_interaction) {
            on_mouse_drag.send({
                OnMouseDrag {
                    phase: EventPhase::Ended,
                    button: *released_button,
                    start: interaction_selector.start_target,
                    end: mouse_interaction.target,
                }
            });
            interaction_selector.reset();
        }
    }
}
