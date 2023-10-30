use bevy::{prelude::*, window::PrimaryWindow};

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
                    test_read_events,
                    update_interaction_ray,
                ),
            );
    }
}

// TODO: MouseTarget struct to save raycast info & coordinates
// TODO: use explicit method ordering

#[derive(Resource)]
pub struct MouseInteraction {
    active_camera: Option<Entity>,
    max_interaction_distance: f32,
    ray_through_cursor: Option<Ray>,
    mouse_on_ui: bool,
}

impl Default for MouseInteraction {
    fn default() -> Self {
        Self {
            active_camera: None,
            max_interaction_distance: 20.0,
            ray_through_cursor: None,
            mouse_on_ui: false,
        }
    }
}

impl MouseInteraction {
    pub fn set_active_camera(&mut self, camera_entity: Entity) {
        self.active_camera = Some(camera_entity);
    }

    pub fn ray_through_cursor(&self) -> Option<Ray> {
        self.ray_through_cursor
    }

    pub fn max_interaction_distance(&self) -> f32 {
        self.max_interaction_distance
    }
}

#[derive(Event, Clone, Copy, Debug)]
pub struct OnMousePressed {
    pub button: MouseButton,
    pub on_ui: bool,
}

fn update_mouse_on_ui(mut mouse_interaction: ResMut<MouseInteraction>, nodes: Query<&Interaction>) {
    mouse_interaction.mouse_on_ui = nodes.iter().any(|interaction| {
        *interaction == Interaction::Hovered || *interaction == Interaction::Pressed
    });
}

fn update_interaction_ray(
    mut mouse_interaction: ResMut<MouseInteraction>,
    mut cameras: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(window) = window.get_single() {
        if let Some(active_camera) = mouse_interaction.active_camera {
            if let Ok((camera, transform)) = cameras.get_mut(active_camera) {
                mouse_interaction.ray_through_cursor = get_cursor_as_ray(camera, transform, window);
            }
        }
    }
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
        });
    }
}

fn test_read_events(mut on_mouse_pressed: EventReader<OnMousePressed>) {
    for event in on_mouse_pressed.iter() {
        println!("{:?}", event);
    }
}

fn get_cursor_as_ray(
    camera: &Camera,
    global_transform: &GlobalTransform,
    window: &Window,
) -> Option<Ray> {
    camera.viewport_to_world(global_transform, window.cursor_position()?)
}
