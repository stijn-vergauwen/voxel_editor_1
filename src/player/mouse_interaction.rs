pub mod mouse_target;

use bevy::prelude::*;

use self::mouse_target::MouseTarget;

pub struct MouseInteractionPlugin;

impl Plugin for MouseInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseInteraction::default())
            .add_event::<OnMousePressed>()
            .add_systems(
                Update,
                (
                    update_mouse_on_ui,
                    send_mouse_pressed_events,
                ),
            );
    }
}

#[derive(Resource)]
pub struct MouseInteraction {
    active_camera: Option<Entity>,
    mouse_on_ui: bool,
    max_interaction_distance: f32,
    ray_through_cursor: Option<Ray>,
    target: Option<MouseTarget>,
}

impl Default for MouseInteraction {
    fn default() -> Self {
        Self {
            active_camera: None,
            mouse_on_ui: false,
            max_interaction_distance: 20.0,
            ray_through_cursor: None,
            target: None,
        }
    }
}

impl MouseInteraction {
    pub fn set_active_camera(&mut self, camera_entity: Entity) {
        self.active_camera = Some(camera_entity);
    }
}


#[derive(Event, Clone, Copy, Debug)]
pub struct OnMousePressed {
    pub button: MouseButton,
    pub on_ui: bool,
    pub target: Option<MouseTarget>,
}

fn update_mouse_on_ui(mut mouse_interaction: ResMut<MouseInteraction>, nodes: Query<&Interaction>) {
    mouse_interaction.mouse_on_ui = nodes.iter().any(|interaction| {
        *interaction == Interaction::Hovered || *interaction == Interaction::Pressed
    });
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