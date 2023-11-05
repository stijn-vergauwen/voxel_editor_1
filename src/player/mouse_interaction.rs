pub mod mouse_events;
pub mod mouse_target;

use bevy::prelude::*;

use self::{
    mouse_events::MouseEventsPlugin,
    mouse_target::{MouseTarget, MouseTargetPlugin},
};

pub struct MouseInteractionPlugin;

impl Plugin for MouseInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MouseTargetPlugin, MouseEventsPlugin))
            .insert_resource(MouseInteraction::default())
            .add_systems(Update, update_mouse_on_ui);
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

impl MouseInteraction {
    pub fn set_active_camera(&mut self, camera_entity: Entity) {
        self.active_camera = Some(camera_entity);
    }
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

fn update_mouse_on_ui(mut mouse_interaction: ResMut<MouseInteraction>, nodes: Query<&Interaction>) {
    mouse_interaction.mouse_on_ui = nodes.iter().any(|interaction| {
        *interaction == Interaction::Hovered || *interaction == Interaction::Pressed
    });
}
